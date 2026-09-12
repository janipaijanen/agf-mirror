//! Local-only projection of publication-hold evidence. No permission to delay.
use agov_coord::{hold_evaluate, hold_git, hold_result::ResultV1};
use std::path::Path;

pub fn run(
    target: &str,
    channel: &str,
    candidates: &[String],
    format: &str,
    args: &[String],
) -> i32 {
    // Embedded, version-matched schema keeps this read-only command independent
    // of working-tree drafts and operator schema overrides.
    let schema = agov_core::schema::compile_str(include_str!(
        "../../../../../agf/schemas/coordination-message.schema.json"
    ))
    .expect("embedded coordination schema must compile");
    let now = time::OffsetDateTime::now_utc();
    let inventory = hold_git::read_inventory(Path::new("."), target, candidates, channel);
    let report = hold_evaluate::evaluate(inventory, channel, &schema, now);
    let output = if format == "json" {
        serde_json::to_string_pretty(&report).expect("hold result is serializable")
    } else {
        render(&report)
    };
    println!("{output}");
    agov_core::roi::record_tool_output(
        "agov",
        args,
        "coord-hold-status",
        output.chars().count() as u64,
    );
    report.exit_status()
}

fn render(r: &ResultV1) -> String {
    let mut lines = vec![
        format!(
            "hold-status target={} sha={} coverage={} scope=explicit-sources now={}",
            r.target_ref.as_deref().unwrap_or("unresolved"),
            r.target_sha.as_deref().unwrap_or("unresolved"),
            r.coverage.as_str(),
            r.now
        ),
        "publication_verified=false authority_verified=false".into(),
        format!(
            "required_external_checks: {}",
            r.required_external_checks
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
    ];
    for s in &r.sources {
        lines.push(format!(
            "source {} refs={} sha={} status={} problems={}",
            s.source_id,
            s.requested_refs.join(","),
            s.sha.as_deref().unwrap_or("unresolved"),
            s.status.as_str(),
            s.problem_ids.join(",")
        ));
    }
    for h in &r.holds {
        lines.push(format!(
            "hold {} temporal={} target={} conflict={} end={} problems={}",
            h.id,
            h.temporal_state.as_str(),
            h.target_state.as_str(),
            h.conflict,
            h.effective_end,
            h.problem_ids.join(",")
        ));
        for e in &h.ends {
            lines.push(format!(
                "  end {} timestamp={} sources={} problems={}",
                e.id,
                e.timestamp,
                e.provenance
                    .iter()
                    .map(|p| format!("{}:{}", p.source_id, p.path))
                    .collect::<Vec<_>>()
                    .join(","),
                e.problem_ids.join(",")
            ));
        }
    }
    for a in &r.records {
        lines.push(format!(
            "record {} state={} urgency={} reasons={} problems={} sources={}",
            a.id.as_deref().unwrap_or("<no-id>"),
            a.state.as_str(),
            a.urgency.as_str(),
            a.reasons
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(","),
            a.problem_ids.join(","),
            a.provenance
                .iter()
                .map(|p| format!("{}:{}", p.source_id, p.path))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    for p in &r.problems {
        lines.push(format!(
            "problem {} {} {}: {}",
            p.problem_id,
            p.code.as_str(),
            p.severity.as_str(),
            p.message
        ));
    }
    lines.push("All rows require the external checks above; publication and authority remain unverified. Review-required never permits delay. Explicit pinned sources are not a whole inbox or proof of fresh publication. If external checks are unavailable, publish immediately. This command does not approve gates.".into());
    lines.join("\n")
}
