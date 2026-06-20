use clap::{Parser, Subcommand};

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
    let cli = Cli::parse_from(normalized_args());
    let exit_code = run(cli);
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

fn run(cli: Cli) -> i32 {
    match cli.command {
        Command::Coord {
            sub: CoordCommand::Validate { files },
        } => validate_files(
            files,
            agov_coord::coord_validator::validate_file,
            Some(agov_coord::coord_validator::advisory_findings),
        ),
        Command::Plan {
            sub: PlanCommand::Validate { files },
        } => validate_files(files, agov_coord::plan_validator::validate_file, None),
        Command::Validate { sub } => run_validate(sub),
    }
}

fn validate_files<T>(
    files: Vec<std::path::PathBuf>,
    validator: FileValidator<T>,
    advisory_fun: Option<AdvisoryFn<T>>,
) -> i32 {
    let mut failed = false;

    for file in files {
        let label = file.to_string_lossy().to_string();
        match validator(&file) {
            Ok(data) => {
                if let Some(advisory_fun) = advisory_fun {
                    for message in advisory_fun(&data) {
                        agov_core::output::coord_warn(&label, &message);
                    }
                }
                agov_core::output::coord_ok(&label);
            }
            Err(errors) => {
                failed = true;
                render_coord_errors(&label, &errors);
            }
        }
    }

    if failed {
        1
    } else {
        0
    }
}

fn render_coord_errors(path: &str, errors: &[(String, String)]) {
    if let [(message, location)] = errors {
        if location.is_empty() {
            agov_core::output::coord_err(path, message);
            return;
        }
    }

    agov_core::output::coord_err_schema(path, errors);
}

fn run_validate(sub: Option<ValidateCommand>) -> i32 {
    let Some(sub) = sub else {
        return run_all_validators();
    };

    match sub {
        ValidateCommand::Adoption => render_collection(
            "adoption",
            agov_validators::adoption::validate_all(&agov_core::repo_root()),
        ),
        ValidateCommand::Agents => render_collection(
            "agents",
            agov_validators::agents::validate_all(
                &agov_core::repo_root().join("agf").join("agents"),
            ),
        ),
        ValidateCommand::Learnings => render_collection(
            "learnings",
            agov_validators::learnings::validate_all(
                &agov_core::repo_root().join("agf").join("learnings"),
            ),
        ),
        ValidateCommand::Infosec => render_collection(
            "infosec",
            agov_validators::infosec::validate_all(
                &agov_core::repo_root().join("agf").join("learnings"),
            ),
        ),
        ValidateCommand::Contributions => render_collection(
            "contributions",
            agov_validators::contributions::validate_all(
                &agov_core::repo_root().join("docs").join("contributions"),
            ),
        ),
        ValidateCommand::FrameworkConfig => render_collection(
            "framework-config",
            agov_validators::framework_config::validate_all(&agov_core::repo_root()),
        ),
    }
}

fn run_all_validators() -> i32 {
    let results = [
        run_validate(Some(ValidateCommand::Adoption)),
        run_validate(Some(ValidateCommand::Agents)),
        run_validate(Some(ValidateCommand::Learnings)),
        run_validate(Some(ValidateCommand::Infosec)),
        run_validate(Some(ValidateCommand::Contributions)),
        run_validate(Some(ValidateCommand::FrameworkConfig)),
    ];

    if results.contains(&1) {
        1
    } else {
        0
    }
}

fn render_collection(label: &str, result: agov_validators::CollectionResult) -> i32 {
    if result.is_ok() {
        agov_core::output::validate_ok(label);
        return 0;
    }

    for file_error in result.file_errors {
        if file_error.findings.is_empty() {
            agov_core::output::validate_err_label(&file_error.file);
        }

        for finding in file_error.findings {
            let location = finding.location.as_deref().unwrap_or(&file_error.file);
            agov_core::output::validate_err(label, location, &finding.message);
        }
    }

    1
}
