use clap::Subcommand;

#[derive(Subcommand)]
pub enum ArtifactCommand {
    /// Validate the complete configured registry and optional requested members.
    Validate {
        /// Repository-relative config; existing coordination keys are preserved.
        #[arg(long, default_value = "agov-config.yaml")]
        config: String,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
        /// Exact repository-relative members; cross-file checks always use all sets.
        paths: Vec<String>,
    },
    /// Emit a deterministic index of all configured artifact sets.
    Index {
        #[arg(long, default_value = "agov-config.yaml")]
        config: String,
        #[arg(long, default_value = "json", value_parser = ["text", "json"])]
        format: String,
    },
}
pub fn run(command: ArtifactCommand, args: &[String]) -> i32 {
    let (config, format, paths) = match command {
        ArtifactCommand::Validate {
            config,
            format,
            paths,
        } => (config, format, paths),
        ArtifactCommand::Index { config, format } => (config, format, vec![]),
    };
    let mut result = agov_validators::artifact_index::inspect(&agov_core::repo_root(), &config);
    result.require_selected_paths(&paths);
    let output = if format == "json" {
        match serde_json::to_string_pretty(&result) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("artifact output failed: {e}");
                return 2;
            }
        }
    } else {
        let mut text = format!(
            "Artifact registry: {:?}, {} files, {} IDs, {} references, {} problems\n",
            result.coverage,
            result.files.len(),
            result.entries.len(),
            result.edges.len(),
            result.problems.len()
        );
        for problem in &result.problems {
            text.push_str(&format!(
                "{} {}{}: {}\n",
                problem.code, problem.path, problem.pointer, problem.message
            ));
        }
        text.push_str("Structural evidence only; this command does not approve governance gates.");
        text
    };
    println!("{output}");
    agov_core::roi::record_tool_output("agov", args, "artifact", output.chars().count() as u64);
    result.exit_status()
}
