use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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
    /// Handoff queue commands.
    Handoff {
        #[command(subcommand)]
        sub: HandoffCommand,
    },
}

#[derive(Subcommand)]
enum CoordCommand {
    /// Validate one or more coordination message files.
    Validate {
        /// Files to validate.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Generate a coordination message skeleton with a new coordination id.
    Brief {
        #[arg(long, default_value = "ulid", value_parser = ["ulid", "readable"])]
        id_format: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "note")]
        kind: String,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
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

#[derive(Subcommand)]
enum HandoffCommand {
    /// Show the next ready handoff for an agent role.
    Next {
        agent_role: String,
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
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
        Command::Coord {
            sub:
                CoordCommand::Brief {
                    id_format,
                    from,
                    to,
                    kind,
                    topic,
                    dir,
                },
        } => coord_brief(id_format, from, to, kind, topic, dir, &args),
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
        Command::Handoff {
            sub: HandoffCommand::Next { agent_role, dir },
        } => handoff_next(&agent_role, dir, &args),
    }
}

fn coord_brief(
    id_format: String,
    from: Option<String>,
    to: Option<String>,
    kind: String,
    topic: Option<String>,
    dir: PathBuf,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();

    if !matches!(
        kind.as_str(),
        "note" | "handoff" | "claim" | "completion" | "ack" | "review"
    ) {
        output.stderr(format!(
            "ERR  unknown kind: {kind}  (supported: note handoff claim completion ack review)"
        ));
        return 1;
    }

    if id_format == "readable" && from.is_none() {
        output.stderr("ERR  --id-format readable requires --from <role>");
        return 1;
    }

    let now = time::OffsetDateTime::now_utc();
    let next_id = if id_format == "readable" {
        readable_id(from.as_deref().unwrap_or("unknown"), now)
    } else {
        agov_core::ulid_util::new()
    };
    let timestamp = if id_format == "readable" {
        iso_timestamp(now)
    } else {
        "<TIMESTAMP>".to_string()
    };
    let from = from.unwrap_or_else(|| "<from>".to_string());
    let to = to.unwrap_or_else(|| "<to>".to_string());
    let topic = topic.unwrap_or_else(|| "<topic>".to_string());
    let coord_dir = repo_relative_path(dir);
    let channel = channel_summary(&coord_dir);

    output.stdout(format!(
        "# Channel: {} messages, {}",
        channel.total, channel.range
    ));
    output.stdout(format!(
        "# Ownership: {} (run mix agov.coord.status for details)",
        channel.claims
    ));
    output.stdout(format!("# Next id: {next_id}"));
    output.stdout("---");
    output.stdout(format!("kind: {kind}"));
    output.stdout(format!("from: {from}"));
    output.stdout(format!("to: {to}"));
    output.stdout(format!("id: {next_id}"));
    output.stdout(format!("timestamp: {timestamp}"));
    output.stdout("immutable: true");
    render_kind_front_matter(&mut output, &kind, &to, &topic);
    output.stdout("---");

    if kind == "review" {
        output.stdout("");
        output.stdout("# <findings — severity: BLOCK | WARN | INFO>");
    } else {
        output.stdout("");
        output.stdout("# <message body>");
    }

    agov_core::roi::record_tool_output("agov", args, "coord-brief", output.chars);
    0
}

struct ChannelSummary {
    total: usize,
    range: String,
    claims: String,
}

fn channel_summary(dir: &Path) -> ChannelSummary {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            return ChannelSummary {
                total: 0,
                range: "(empty)".to_string(),
                claims: "no active claims".to_string(),
            }
        }
    };

    let mut timestamps = Vec::new();
    let mut total = 0usize;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        if let Ok(message) = agov_coord::coord_validator::validate_file(&path) {
            total += 1;
            if let Some(timestamp) = message
                .raw
                .get("timestamp")
                .and_then(|value| value.as_str())
                .map(str::to_string)
            {
                timestamps.push(timestamp);
            }
        }
    }

    timestamps.sort();
    let range = match (timestamps.first(), timestamps.last()) {
        (Some(first), Some(last)) => format!("{first} to {last}"),
        _ => "(empty)".to_string(),
    };

    ChannelSummary {
        total,
        range,
        claims: "no active claims".to_string(),
    }
}

fn repo_relative_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        agov_core::repo_root().join(path)
    }
}

fn render_kind_front_matter(output: &mut OutputCounter, kind: &str, to: &str, topic: &str) {
    match kind {
        "handoff" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
            output.stdout("handoff:");
            output.stdout(format!("  next_agent: {to}"));
            output.stdout("  status: ready");
            output.stdout("  task: []");
            output.stdout("  must_read: []");
            output.stdout("  do_not_touch:");
            output.stdout("    - .claude/settings.local.json");
            output.stdout("    - tooling/elixir/mix.lock");
        }
        "claim" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned:");
            output.stdout("  # list paths you are claiming ownership of");
            output.stdout("  - <path>");
        }
        "completion" => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned:");
            output.stdout("  # list paths being released; [] releases all by this sender");
            output.stdout("  []");
        }
        "ack" => {
            output.stdout("references-to:");
            output.stdout("  # filename(s) this ACK acknowledges");
            output.stdout("  - <filename>");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
        }
        _ => {
            output.stdout("references-to: []");
            output.stdout(format!("topic: {topic}"));
            output.stdout("requires-ack: false");
            output.stdout("artifacts_owned: []");
        }
    }
}

fn readable_id(role: &str, now: time::OffsetDateTime) -> String {
    format!(
        "{}-{}-{}",
        basic_utc_timestamp(now),
        role_slug(role),
        readable_suffix()
    )
}

fn basic_utc_timestamp(now: time::OffsetDateTime) -> String {
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

fn iso_timestamp(now: time::OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

fn role_slug(role: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for ch in role.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "unknown".to_string()
    } else {
        slug
    }
}

fn readable_suffix() -> String {
    let alphabet = b"0123456789abcdefghjkmnpqrstvwxyz";
    let ulid = agov_core::ulid_util::new();

    ulid.bytes()
        .rev()
        .take(4)
        .map(|byte| alphabet[(byte as usize) % alphabet.len()] as char)
        .collect()
}

#[derive(Clone)]
struct HandoffMessage {
    file: String,
    sort_key: (String, String),
    raw: serde_json::Value,
}

fn handoff_next(agent_role: &str, dir: PathBuf, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!(
                "Coordination directory not found: {}",
                dir.display()
            ));
            return 1;
        }
    };

    let pending = pending_handoffs(&messages);
    let next = pending
        .into_iter()
        .filter(|message| string_field(&message.raw, "to") == Some(agent_role))
        .max_by_key(|message| message.sort_key.clone());

    let Some(message) = next else {
        output.stdout(format!("No handoff found for role: {agent_role}"));
        return 2;
    };

    render_handoff_summary(&mut output, &message);
    agov_core::roi::record_tool_output(
        "agov.handoff.next",
        args,
        &format!("next-handoff:{agent_role}"),
        output.chars,
    );
    0
}

fn read_valid_coord_messages(dir: &Path) -> Result<Vec<HandoffMessage>, std::io::Error> {
    let mut messages = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let Ok(message) = agov_coord::coord_validator::validate_file(&path) else {
            continue;
        };
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        let timestamp = string_field(&message.raw, "timestamp")
            .unwrap_or("")
            .to_string();
        let file = file_name.to_string();
        messages.push(HandoffMessage {
            sort_key: (timestamp, file.clone()),
            file,
            raw: message.raw,
        });
    }

    messages.sort_by_key(|message| message.sort_key.clone());
    Ok(messages)
}

fn pending_handoffs(messages: &[HandoffMessage]) -> Vec<HandoffMessage> {
    let mut resolved_files = std::collections::BTreeSet::new();

    for message in messages {
        let kind = string_field(&message.raw, "kind");
        if !matches!(kind, Some("ack" | "completion" | "decision-record")) {
            continue;
        }

        for referenced in references_to(&message.raw) {
            resolved_files.insert(referenced);
        }
    }

    messages
        .iter()
        .filter(|message| string_field(&message.raw, "kind") == Some("handoff"))
        .filter(|message| !resolved_files.contains(&message.file))
        .cloned()
        .collect()
}

fn references_to(raw: &serde_json::Value) -> Vec<String> {
    raw.get("references-to")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .filter_map(|reference| Path::new(reference).file_name()?.to_str())
        .map(str::to_string)
        .collect()
}

fn render_handoff_summary(output: &mut OutputCounter, message: &HandoffMessage) {
    let handoff = message
        .raw
        .get("handoff")
        .and_then(|value| value.as_object());
    let next_agent = handoff
        .and_then(|value| value.get("next_agent"))
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| string_field(&message.raw, "to").unwrap_or("unknown"));
    let status = handoff
        .and_then(|value| value.get("status"))
        .and_then(|value| value.as_str())
        .unwrap_or("ready");

    output.stdout(format!("Handoff for: {next_agent}"));
    output.stdout(format!("Status:      {status}"));
    output.stdout("Task:");
    render_string_list(output, handoff, "task");
    render_titled_string_list(output, handoff, "Must read:", "must_read");
    render_titled_string_list(output, handoff, "Do not touch:", "do_not_touch");
    output.stdout(format!("Source: {}", message.file));
}

fn render_titled_string_list(
    output: &mut OutputCounter,
    handoff: Option<&serde_json::Map<String, serde_json::Value>>,
    title: &str,
    field: &str,
) {
    let Some(items) = handoff
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_array())
        .filter(|items| !items.is_empty())
    else {
        return;
    };

    output.stdout(title);
    for item in items.iter().filter_map(|item| item.as_str()) {
        output.stdout(format!("  - {item}"));
    }
}

fn render_string_list(
    output: &mut OutputCounter,
    handoff: Option<&serde_json::Map<String, serde_json::Value>>,
    field: &str,
) {
    if let Some(items) = handoff
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_array())
    {
        for item in items.iter().filter_map(|item| item.as_str()) {
            output.stdout(format!("  - {item}"));
        }
    }
}

fn string_field<'a>(raw: &'a serde_json::Value, field: &str) -> Option<&'a str> {
    raw.get(field).and_then(|value| value.as_str())
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
