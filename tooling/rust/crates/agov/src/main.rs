use clap::{Parser, Subcommand};
use std::collections::BTreeMap;

type ValidationErrors = Vec<(String, String)>;
type FileValidator<T> = fn(&std::path::Path) -> Result<T, ValidationErrors>;
type AdvisoryFn<T> = fn(&T) -> Vec<String>;

#[derive(Parser)]
#[command(name = "agov", about = "AGF governance tooling")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Coordination channel commands.
    Coord {
        #[command(subcommand)]
        sub: CoordCommand,
    },
    /// Gate 1 plan commands.
    Plan {
        #[command(subcommand)]
        sub: PlanCommand,
    },
    /// Run validators.
    Validate {
        #[command(subcommand)]
        sub: Option<ValidateCommand>,
    },
}

#[derive(Subcommand)]
enum CoordCommand {
    /// Validate one or more coordination message files.
    Validate {
        /// Files to validate.
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
    // Phase 2: Status, Stale
}

#[derive(Subcommand)]
enum PlanCommand {
    /// Validate one or more Gate 1 plan files.
    Validate {
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum ValidateCommand {
    /// Validate the adoption structure.
    Adoption,
    /// Validate all agent YAML files.
    Agents,
    /// Validate all learning records.
    Learnings,
    /// Scan learning records for InfoSec findings.
    Infosec,
    /// Validate contribution-auditor findings.
    Contributions,
    /// Validate framework-config.
    FrameworkConfig,
}

fn main() {
    let args = normalized_args();
    let cli = Cli::parse_from(args.clone());
    let exit_code = run(cli, args.into_iter().skip(1).collect());
    std::process::exit(exit_code);
}

fn normalized_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();
    if let Some(command) = args.get_mut(1) {
        match command.as_str() {
            "agov.coord.validate" => {
                *command = "coord".to_string();
                args.insert(2, "validate".to_string());
            }
            "agov.plan.validate" => {
                *command = "plan".to_string();
                args.insert(2, "validate".to_string());
            }
            _ => {}
        }
    }
    args
}

fn run(cli: Cli, args: Vec<String>) -> i32 {
    match cli.command {
        Command::Coord {
            sub: CoordCommand::Validate { files },
        } => validate_coord_files(files, &args),
        Command::Plan {
            sub: PlanCommand::Validate { files },
        } => validate_files(
            files,
            agov_coord::plan_validator::validate_file,
            None,
            &args,
            "plan-validate",
        ),
        Command::Validate { sub } => run_validate(sub, &args),
    }
}

fn validate_coord_files(files: Vec<std::path::PathBuf>, args: &[String]) -> i32 {
    let mut failed = false;
    let mut output = OutputCounter::default();
    let mut valid_messages = Vec::new();

    for file in files {
        let label = file.to_string_lossy().to_string();
        match agov_coord::coord_validator::validate_file(&file) {
            Ok(data) => {
                for message in
                    agov_coord::coord_validator::advisory_findings_for_path(&data, Some(&file))
                {
                    output.coord_warn(&label, &message);
                }
                output.coord_ok(&label);
                valid_messages.push((label, data));
            }
            Err(errors) => {
                failed = true;
                render_coord_errors(&mut output, &label, &errors);
            }
        }
    }

    for (path, message) in duplicate_id_errors(&valid_messages) {
        failed = true;
        output.coord_err(&path, &message);
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "coord-validate", output.chars);
        0
    }
}

fn duplicate_id_errors(
    messages: &[(String, agov_coord::coord_validator::CoordMessage)],
) -> Vec<(String, String)> {
    let mut by_id: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (path, message) in messages {
        if let Some(id) = message.id() {
            by_id.entry(id.to_string()).or_default().push(path.clone());
        }
    }

    let mut errors = Vec::new();
    for (id, mut paths) in by_id {
        paths.sort();
        if let Some(first) = paths.first().cloned() {
            for path in paths.into_iter().skip(1) {
                errors.push((
                    path,
                    format!("duplicate coordination id '{id}' also used by {first}"),
                ));
            }
        }
    }

    errors
}

fn validate_files<T>(
    files: Vec<std::path::PathBuf>,
    validator: FileValidator<T>,
    advisory_fun: Option<AdvisoryFn<T>>,
    args: &[String],
    workflow: &str,
) -> i32 {
    let mut failed = false;
    let mut output = OutputCounter::default();

    for file in files {
        let label = file.to_string_lossy().to_string();
        match validator(&file) {
            Ok(data) => {
                if let Some(advisory_fun) = advisory_fun {
                    for message in advisory_fun(&data) {
                        output.coord_warn(&label, &message);
                    }
                }
                output.coord_ok(&label);
            }
            Err(errors) => {
                failed = true;
                render_coord_errors(&mut output, &label, &errors);
            }
        }
    }

    if failed {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, workflow, output.chars);
        0
    }
}

fn render_coord_errors(output: &mut OutputCounter, path: &str, errors: &[(String, String)]) {
    if let [(message, location)] = errors {
        if location.is_empty() {
            output.coord_err(path, message);
            return;
        }
    }

    output.coord_err_schema(path, errors);
}

fn run_validate(sub: Option<ValidateCommand>, args: &[String]) -> i32 {
    let Some(sub) = sub else {
        return run_all_validators(args);
    };

    let mut output = OutputCounter::default();
    let (exit_code, workflow) = match sub {
        ValidateCommand::Adoption => (
            render_single_collection(
                &mut output,
                "agov.validate.adoption",
                agov_validators::adoption::validate_all(&agov_core::repo_root().join("agf")),
            ),
            "validate:adoption",
        ),
        ValidateCommand::Agents => (
            render_collection(
                &mut output,
                "agov.validate.agents",
                agov_validators::agents::validate_all(
                    &agov_core::repo_root().join("agf").join("agents"),
                ),
            ),
            "validate:agents",
        ),
        ValidateCommand::Learnings => (
            render_collection(
                &mut output,
                "agov.validate.learnings",
                agov_validators::learnings::validate_all(
                    &agov_core::repo_root().join("agf").join("learnings"),
                ),
            ),
            "validate:learnings",
        ),
        ValidateCommand::Infosec => (
            render_collection(
                &mut output,
                "agov.validate.infosec",
                agov_validators::infosec::validate_all(
                    &agov_core::repo_root().join("agf").join("learnings"),
                ),
            ),
            "validate:infosec",
        ),
        ValidateCommand::Contributions => (
            render_warning_collection(
                &mut output,
                "agov.validate.contributions",
                agov_validators::contributions::validate_all(
                    &agov_core::repo_root().join("agf").join("learnings"),
                ),
            ),
            "validate:contributions",
        ),
        ValidateCommand::FrameworkConfig => {
            let root = agov_core::repo_root();
            if !root.join("agf").join("framework.yaml").exists() {
                output.stdout("OK agov.validate.framework_config: skipped");
                (0, "validate:framework-config")
            } else {
                (
                    render_single_collection(
                        &mut output,
                        "agov.validate.framework_config",
                        agov_validators::framework_config::validate_all(&root.join("agf")),
                    ),
                    "validate:framework-config",
                )
            }
        }
    };

    if exit_code == 0 {
        agov_core::roi::record_tool_output("agov", args, workflow, output.chars);
    }

    exit_code
}

fn run_all_validators(args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let results = [
        render_collection(
            &mut output,
            "agov.validate.learnings",
            agov_validators::learnings::validate_all(
                &agov_core::repo_root().join("agf").join("learnings"),
            ),
        ),
        render_collection(
            &mut output,
            "agov.validate.agents",
            agov_validators::agents::validate_all(
                &agov_core::repo_root().join("agf").join("agents"),
            ),
        ),
        render_single_collection(
            &mut output,
            "agov.validate.adoption",
            agov_validators::adoption::validate_all(&agov_core::repo_root().join("agf")),
        ),
        render_collection(
            &mut output,
            "agov.validate.infosec",
            agov_validators::infosec::validate_all(
                &agov_core::repo_root().join("agf").join("learnings"),
            ),
        ),
        render_warning_collection(
            &mut output,
            "agov.validate.contributions",
            agov_validators::contributions::validate_all(
                &agov_core::repo_root().join("agf").join("learnings"),
            ),
        ),
        render_framework_config(&mut output),
    ];

    if results.contains(&1) {
        1
    } else {
        agov_core::roi::record_tool_output("agov", args, "validate:umbrella", output.chars);
        0
    }
}

fn render_framework_config(output: &mut OutputCounter) -> i32 {
    let root = agov_core::repo_root();
    if !root.join("agf").join("framework.yaml").exists() {
        output.stdout("OK agov.validate.framework_config: skipped");
        0
    } else {
        render_single_collection(
            output,
            "agov.validate.framework_config",
            agov_validators::framework_config::validate_all(&root.join("agf")),
        )
    }
}

fn render_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    if result.is_ok() {
        for file in result.ok_files {
            output.validate_ok(&file);
        }
        output.validate_ok(label);
        return 0;
    }

    for file_error in result.file_errors {
        if file_error.findings.is_empty() {
            output.validate_err_label(&file_error.file);
        }

        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            output.validate_err(label, location, &finding.message);
        }
    }

    1
}

fn render_single_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    if result.is_ok() {
        output.validate_ok(label);
        return 0;
    }

    for file_error in result.file_errors {
        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            output.validate_err(label, location, &finding.message);
        }
    }

    1
}

fn render_warning_collection(
    output: &mut OutputCounter,
    label: &str,
    result: agov_validators::CollectionResult,
) -> i32 {
    for file_error in result.file_errors {
        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            output.validate_warn(&file_error.file, location, &finding.message);
        }
    }

    output.validate_ok(label);
    0
}

#[derive(Default)]
struct OutputCounter {
    chars: u64,
}

impl OutputCounter {
    fn stdout(&mut self, line: impl AsRef<str>) {
        let line = line.as_ref();
        println!("{line}");
        self.count_line(line);
    }

    fn stderr(&mut self, line: impl AsRef<str>) {
        let line = line.as_ref();
        eprintln!("{line}");
        self.count_line(line);
    }

    fn coord_ok(&mut self, path: &str) {
        self.stdout(format!("OK  severity: blocking {path}"));
    }

    fn coord_warn(&mut self, path: &str, msg: &str) {
        self.stdout(format!("WARN severity: advisory {path}: {msg}"));
    }

    fn coord_err(&mut self, path: &str, msg: &str) {
        self.stderr(format!("ERR severity: blocking {path}: {msg}"));
    }

    fn coord_err_schema(&mut self, path: &str, errors: &[(String, String)]) {
        self.stderr(format!("ERR severity: blocking {path}:"));
        for (msg, ptr) in errors {
            self.stderr(format!("    {ptr}: {msg}"));
        }
    }

    fn validate_ok(&mut self, label: &str) {
        self.stdout(format!("OK {label}"));
    }

    fn validate_warn(&mut self, label: &str, location: &str, message: &str) {
        self.stdout(format!("WARN {label}: {location}: {message}"));
    }

    fn validate_err(&mut self, label: &str, location: &str, message: &str) {
        self.stderr(format!("ERR {label}: {location}: {message}"));
    }

    fn validate_err_label(&mut self, label: &str) {
        self.stderr(format!("ERR {label}"));
    }

    fn count_line(&mut self, line: &str) {
        self.chars += line.chars().count() as u64 + 1;
    }
}
