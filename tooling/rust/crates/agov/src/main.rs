use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;

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
    /// Print coordination channel overview.
    Status {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
        #[arg(long, default_value_t = 5)]
        recent: usize,
        /// Include per-role activity derived from manifest + coordination messages.
        #[arg(long)]
        roles: bool,
    },
    /// Report stale artifact claims in the coordination channel.
    Stale {
        #[arg(long, default_value = "docs/coordination")]
        dir: PathBuf,
    },
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
        Command::Coord {
            sub: CoordCommand::Status { dir, recent, roles },
        } => coord_status(dir, recent, roles, &args),
        Command::Coord {
            sub: CoordCommand::Stale { dir },
        } => coord_stale(dir, &args),
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
    let suggested_filename = suggested_filename(&id_format, from.as_deref(), &next_id);
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
    output.stdout(format!("# Suggested file: {suggested_filename}"));
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

fn suggested_filename(id_format: &str, from: Option<&str>, id: &str) -> String {
    if id_format == "readable" {
        format!("{id}.md")
    } else if let Some(from) = from {
        format!("{id}-{from}.md")
    } else {
        format!("{id}-<from>.md")
    }
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
    refs: Vec<String>,
    raw: serde_json::Value,
}

fn coord_status(dir: PathBuf, recent: usize, roles: bool, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 1;
        }
    };

    let display_dir = display_coord_dir(&dir);
    if messages.is_empty() {
        output.stdout(format!("Channel: {display_dir}  (empty)"));
        agov_core::roi::record_tool_output("agov", args, "coord-status", output.chars);
        return 0;
    }

    output.stdout(format!(
        "Channel: {display_dir}  ({} messages, {})",
        messages.len(),
        message_range(&messages)
    ));
    output.stdout("");
    output.stdout("Ownership:    none");
    output.stdout("");
    output.stdout("Stale claims: none");
    output.stdout("");

    let pending = pending_handoffs(&messages);
    if pending.is_empty() {
        output.stdout("Pending handoffs: none");
    } else {
        output.stdout("Pending handoffs:");
        for message in pending.iter().rev() {
            output.stdout(format!(
                "  → {}  {}  {}  [{}]",
                string_field(&message.raw, "to").unwrap_or("unknown"),
                display_id(&message.raw),
                string_field(&message.raw, "topic").unwrap_or(""),
                message
                    .raw
                    .get("handoff")
                    .and_then(|value| value.get("status"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("ready")
            ));
        }
    }
    output.stdout("");

    if recent > 0 {
        output.stdout(format!("Recent messages (last {recent}):"));
        for message in messages.iter().rev().take(recent) {
            output.stdout(format!(
                "  {}  {} → {}  {:<14}  {}",
                display_id(&message.raw),
                string_field(&message.raw, "from").unwrap_or("unknown"),
                string_field(&message.raw, "to").unwrap_or("unknown"),
                string_field(&message.raw, "kind").unwrap_or("unknown"),
                string_field(&message.raw, "topic").unwrap_or("")
            ));
        }
    }

    if roles {
        if recent > 0 {
            output.stdout("");
        }
        render_role_activity(&mut output, &messages);
    }

    agov_core::roi::record_tool_output("agov", args, "coord-status", output.chars);
    0
}

fn display_coord_dir(dir: &Path) -> String {
    let root = agov_core::repo_root();
    dir.strip_prefix(&root)
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| dir.display().to_string())
}

fn message_range(messages: &[HandoffMessage]) -> String {
    let first = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .min();
    let last = messages
        .iter()
        .filter_map(|message| string_field(&message.raw, "timestamp"))
        .max();

    match (first, last) {
        (Some(first), Some(last)) => format!("{first} to {last}"),
        _ => "(empty)".to_string(),
    }
}

fn display_id(raw: &serde_json::Value) -> String {
    if let Some(id) = string_field(raw, "id") {
        format!("id {}", id.chars().take(8).collect::<String>())
    } else if let Some(seq) = raw.get("seq").and_then(|value| value.as_i64()) {
        format!("seq {seq}")
    } else {
        "id <missing>".to_string()
    }
}

#[derive(Default)]
struct RoleActivity {
    status: String,
    from_count: usize,
    to_count: usize,
    last_seen: Option<(String, String)>,
}

fn render_role_activity(output: &mut OutputCounter, messages: &[HandoffMessage]) {
    let mut roles = manifest_roles();

    for message in messages {
        for field in ["from", "to"] {
            let Some(role) = string_field(&message.raw, field) else {
                continue;
            };
            roles
                .entry(role.to_string())
                .or_insert_with(|| RoleActivity {
                    status: "virtual".to_string(),
                    ..RoleActivity::default()
                });
            let entry = roles.get_mut(role).expect("role entry was just inserted");
            if field == "from" {
                entry.from_count += 1;
            } else {
                entry.to_count += 1;
            }
            if let Some(timestamp) = string_field(&message.raw, "timestamp") {
                let candidate = (timestamp.to_string(), message.file.clone());
                if entry
                    .last_seen
                    .as_ref()
                    .map(|current| candidate > *current)
                    .unwrap_or(true)
                {
                    entry.last_seen = Some(candidate);
                }
            }
        }
    }

    output.stdout("Role activity:");
    for (role, activity) in roles {
        let last_seen = activity
            .last_seen
            .as_ref()
            .map(|(timestamp, file)| format!("{timestamp} ({file})"))
            .unwrap_or_else(|| "never".to_string());
        output.stdout(format!(
            "  {role}  status: {}  from: {}  to: {}  last-seen: {last_seen}",
            activity.status, activity.from_count, activity.to_count
        ));
    }
}

fn manifest_roles() -> BTreeMap<String, RoleActivity> {
    let Ok(content) = std::fs::read_to_string(agov_core::manifest_path()) else {
        return BTreeMap::new();
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        return BTreeMap::new();
    };
    let Some(agents) = data.get("agents").and_then(|value| value.as_object()) else {
        return BTreeMap::new();
    };

    agents
        .iter()
        .map(|(role, attrs)| {
            let status = attrs
                .get("status")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
                .to_string();
            (
                role.clone(),
                RoleActivity {
                    status,
                    ..RoleActivity::default()
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct ClaimEntry {
    path: String,
    from: String,
    seq: Option<i64>,
    id: Option<String>,
    claimed_at: String,
    expires_on: Option<time::OffsetDateTime>,
    expires_raw: Option<String>,
}

#[derive(Debug, Clone)]
struct InvalidExpiresOn {
    file: String,
    value: String,
}

fn coord_stale(dir: PathBuf, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);

    let messages = match read_valid_coord_messages(&dir) {
        Ok(messages) => messages,
        Err(_) => {
            output.stderr(format!("ERR  directory not found: {}", dir.display()));
            return 2;
        }
    };

    let aliases = load_from_aliases(&dir);
    let now = time::OffsetDateTime::now_utc();
    let stale = match stale_claims(&messages, &aliases, now) {
        Ok(stale) => stale,
        Err(errors) => {
            for error in errors {
                output.stderr(format!(
                    "ERR  {}  invalid expires_on: {}",
                    error.file, error.value
                ));
            }
            return 2;
        }
    };

    if stale.is_empty() {
        output.stdout("OK  no stale claims");
        agov_core::roi::record_tool_output("agov", args, "coord-stale", output.chars);
        return 0;
    }

    for entry in stale {
        output.stdout(format!(
            "STALE  {}  claimed-by: {}  seq: {}  claimed-at: {}  expired-at: {}",
            entry.path,
            entry.from,
            claim_seq_label(&entry),
            entry.claimed_at,
            entry.expires_raw.unwrap_or_default()
        ));
    }
    1
}

fn stale_claims(
    messages: &[HandoffMessage],
    aliases: &BTreeMap<String, String>,
    now: time::OffsetDateTime,
) -> Result<Vec<ClaimEntry>, Vec<InvalidExpiresOn>> {
    let mut states: BTreeMap<String, ClaimEntry> = BTreeMap::new();
    let mut errors = Vec::new();

    for message in messages {
        match string_field(&message.raw, "kind") {
            Some("claim") => {
                let expires_raw = string_field(&message.raw, "expires_on").map(str::to_string);
                let expires_on = match expires_raw.as_deref() {
                    Some(value) => match time::OffsetDateTime::parse(value, &Rfc3339) {
                        Ok(parsed) => Some(parsed),
                        Err(_) => {
                            errors.push(InvalidExpiresOn {
                                file: message.file.clone(),
                                value: value.to_string(),
                            });
                            continue;
                        }
                    },
                    None => None,
                };
                let from = string_field(&message.raw, "from")
                    .unwrap_or("unknown")
                    .to_string();
                let seq = message.raw.get("seq").and_then(|value| value.as_i64());
                let id = string_field(&message.raw, "id").map(str::to_string);
                let claimed_at = string_field(&message.raw, "timestamp")
                    .unwrap_or("")
                    .to_string();

                for path in string_array_field(&message.raw, "artifacts_owned") {
                    states.insert(
                        path.clone(),
                        ClaimEntry {
                            path,
                            from: from.clone(),
                            seq,
                            id: id.clone(),
                            claimed_at: claimed_at.clone(),
                            expires_on,
                            expires_raw: expires_raw.clone(),
                        },
                    );
                }
            }
            Some("completion") => release_claims(&mut states, &message.raw, aliases),
            _ => {}
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(states
        .into_values()
        .filter(|entry| {
            entry
                .expires_on
                .map(|expires| expires < now)
                .unwrap_or(false)
        })
        .collect())
}

fn release_claims(
    states: &mut BTreeMap<String, ClaimEntry>,
    raw: &serde_json::Value,
    aliases: &BTreeMap<String, String>,
) {
    let from = resolve_alias(string_field(raw, "from").unwrap_or("unknown"), aliases);
    let paths = string_array_field(raw, "artifacts_owned");

    if paths.is_empty() {
        states.retain(|_path, entry| resolve_alias(&entry.from, aliases) != from);
    } else {
        for path in paths {
            let should_release = states
                .get(&path)
                .map(|entry| resolve_alias(&entry.from, aliases) == from)
                .unwrap_or(false);
            if should_release {
                states.remove(&path);
            }
        }
    }
}

fn claim_seq_label(entry: &ClaimEntry) -> String {
    entry
        .seq
        .map(|seq| seq.to_string())
        .or_else(|| entry.id.as_ref().map(|_id| String::new()))
        .unwrap_or_default()
}

fn load_from_aliases(dir: &Path) -> BTreeMap<String, String> {
    let path = dir.join("agov-config.yaml");
    let Ok(content) = std::fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    let Ok(data) = serde_norway::from_str::<serde_json::Value>(&content) else {
        return BTreeMap::new();
    };
    let Some(aliases) = data.get("from_aliases").and_then(|value| value.as_object()) else {
        return BTreeMap::new();
    };

    aliases
        .iter()
        .filter_map(|(from, to)| Some((from.clone(), to.as_str()?.to_string())))
        .collect()
}

fn resolve_alias(value: &str, aliases: &BTreeMap<String, String>) -> String {
    aliases
        .get(value)
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

fn string_array_field(raw: &serde_json::Value, field: &str) -> Vec<String> {
    raw.get(field)
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .map(str::to_string)
        .collect()
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
        let refs = message_refs(&path, &message.raw);
        messages.push(HandoffMessage {
            sort_key: (timestamp, file.clone()),
            file,
            refs,
            raw: message.raw,
        });
    }

    messages.sort_by_key(|message| message.sort_key.clone());
    Ok(messages)
}

fn pending_handoffs(messages: &[HandoffMessage]) -> Vec<HandoffMessage> {
    let descendants_by_file = descendants_by_file(messages);
    let messages_by_file: BTreeMap<&str, &HandoffMessage> = messages
        .iter()
        .map(|message| (message.file.as_str(), message))
        .collect();

    messages
        .iter()
        .filter(|message| string_field(&message.raw, "kind") == Some("handoff"))
        .filter(|message| !handoff_resolved(message, &descendants_by_file, &messages_by_file))
        .cloned()
        .collect()
}

fn handoff_resolved(
    handoff: &HandoffMessage,
    descendants_by_file: &BTreeMap<String, Vec<String>>,
    messages_by_file: &BTreeMap<&str, &HandoffMessage>,
) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = vec![handoff.file.clone()];

    while let Some(file) = stack.pop() {
        if !seen.insert(file.clone()) {
            continue;
        }

        for descendant in descendants_by_file.get(&file).into_iter().flatten() {
            if let Some(message) = messages_by_file.get(descendant.as_str()) {
                if message.sort_key > handoff.sort_key
                    && matches!(
                        string_field(&message.raw, "kind"),
                        Some("ack" | "completion" | "decision-record")
                    )
                {
                    return true;
                }
                stack.push(descendant.clone());
            }
        }
    }

    false
}

fn descendants_by_file(messages: &[HandoffMessage]) -> BTreeMap<String, Vec<String>> {
    let messages_by_file: BTreeMap<&str, &HandoffMessage> = messages
        .iter()
        .map(|message| (message.file.as_str(), message))
        .collect();
    let mut descendants: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for message in messages {
        for referenced in &message.refs {
            let Some(referenced_message) = messages_by_file.get(referenced.as_str()) else {
                continue;
            };
            if message.sort_key <= referenced_message.sort_key {
                continue;
            }

            descendants
                .entry(referenced.clone())
                .or_default()
                .push(message.file.clone());
        }
    }

    descendants
}

fn message_refs(path: &Path, raw: &serde_json::Value) -> Vec<String> {
    let mut refs = frontmatter_refs(raw);
    refs.extend(body_refs(path));
    refs.sort();
    refs.dedup();
    refs
}

fn frontmatter_refs(raw: &serde_json::Value) -> Vec<String> {
    raw.get("references-to")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .filter_map(|reference| Path::new(reference).file_name()?.to_str())
        .map(str::to_string)
        .collect()
}

fn body_refs(path: &Path) -> Vec<String> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let body = content.splitn(3, "---").nth(2).unwrap_or(content.as_str());

    body.split(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | '(' | ')' | ','))
        .filter_map(clean_coord_filename)
        .collect()
}

fn clean_coord_filename(token: &str) -> Option<String> {
    let filename = token
        .trim_matches(|ch: char| matches!(ch, '.' | ':' | ';' | ']' | '[' | ')' | '('))
        .rsplit('/')
        .next()?;

    if filename.ends_with(".md") && looks_like_coord_filename(filename) {
        Some(filename.to_string())
    } else {
        None
    }
}

fn looks_like_coord_filename(filename: &str) -> bool {
    let stem = filename.strip_suffix(".md").unwrap_or(filename);
    let first = stem.split('-').next().unwrap_or("");

    first.len() == 5 && first.chars().all(|ch| ch.is_ascii_digit())
        || first.len() == 26
            && first
                .chars()
                .all(|ch| matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z'))
        || stem.len() >= 17
            && stem.as_bytes().get(8) == Some(&b'T')
            && stem.as_bytes().get(15) == Some(&b'Z')
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

#[cfg(test)]
mod tests {
    use super::*;

    fn message(file: &str, kind: &str, timestamp: &str, refs: &[&str]) -> HandoffMessage {
        HandoffMessage {
            file: file.to_string(),
            sort_key: (timestamp.to_string(), file.to_string()),
            refs: refs.iter().map(|value| value.to_string()).collect(),
            raw: serde_json::json!({
                "kind": kind,
                "from": "coder-tester",
                "to": "governor-reviewer",
                "topic": "test",
                "timestamp": timestamp,
                "id": file.trim_end_matches(".md"),
                "handoff": {
                    "status": "ready"
                }
            }),
        }
    }

    #[test]
    fn pending_handoffs_follow_transitive_resolution_refs() {
        let handoff = message(
            "00001-coder-tester.md",
            "handoff",
            "2026-06-21T00:00:00Z",
            &[],
        );
        let note = message(
            "00002-planner-architect.md",
            "note",
            "2026-06-21T00:01:00Z",
            &["00001-coder-tester.md"],
        );
        let completion = message(
            "00003-coder-tester.md",
            "completion",
            "2026-06-21T00:02:00Z",
            &["00002-planner-architect.md"],
        );

        let pending = pending_handoffs(&[handoff, note, completion]);

        assert!(pending.is_empty());
    }

    fn claim_message(
        file: &str,
        from: &str,
        seq: i64,
        path: &str,
        expires_on: Option<&str>,
    ) -> HandoffMessage {
        let mut raw = serde_json::json!({
            "kind": "claim",
            "from": from,
            "to": "all",
            "topic": "test",
            "timestamp": "2026-01-01T00:00:00Z",
            "seq": seq,
            "id": file.trim_end_matches(".md"),
            "artifacts_owned": [path],
        });
        if let Some(expires_on) = expires_on {
            raw["expires_on"] = serde_json::Value::String(expires_on.to_string());
        }

        HandoffMessage {
            file: file.to_string(),
            sort_key: ("2026-01-01T00:00:00Z".to_string(), file.to_string()),
            refs: Vec::new(),
            raw,
        }
    }

    fn completion_message(file: &str, from: &str, paths: Vec<&str>) -> HandoffMessage {
        HandoffMessage {
            file: file.to_string(),
            sort_key: ("2026-01-01T00:01:00Z".to_string(), file.to_string()),
            refs: Vec::new(),
            raw: serde_json::json!({
                "kind": "completion",
                "from": from,
                "to": "all",
                "topic": "test",
                "timestamp": "2026-01-01T00:01:00Z",
                "id": file.trim_end_matches(".md"),
                "artifacts_owned": paths,
            }),
        }
    }

    #[test]
    fn stale_claims_reports_expired_unreleased_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("2025-01-01T00:00:00Z"),
        );

        let stale = stale_claims(&[claim], &BTreeMap::new(), now).unwrap();

        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].path, "docs/stale.md");
        assert_eq!(stale[0].seq, Some(10));
    }

    #[test]
    fn stale_claims_ignores_active_and_non_expiring_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let active = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/active.md",
            Some("2099-01-01T00:00:00Z"),
        );
        let no_expiry = claim_message(
            "00011-coder-tester.md",
            "coder-tester",
            11,
            "docs/no-expiry.md",
            None,
        );

        let stale = stale_claims(&[active, no_expiry], &BTreeMap::new(), now).unwrap();

        assert!(stale.is_empty());
    }

    #[test]
    fn stale_claims_excludes_released_claims() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("2025-01-01T00:00:00Z"),
        );
        let completion = completion_message("00011-coder-tester.md", "coder-tester", vec![]);

        let stale = stale_claims(&[claim, completion], &BTreeMap::new(), now).unwrap();

        assert!(stale.is_empty());
    }

    #[test]
    fn stale_claims_rejects_invalid_expires_on() {
        let now = time::OffsetDateTime::parse("2026-01-01T00:00:00Z", &Rfc3339).unwrap();
        let claim = claim_message(
            "00010-coder-tester.md",
            "coder-tester",
            10,
            "docs/stale.md",
            Some("not-a-date"),
        );

        let errors = stale_claims(&[claim], &BTreeMap::new(), now).unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].file, "00010-coder-tester.md");
        assert_eq!(errors[0].value, "not-a-date");
    }
}
