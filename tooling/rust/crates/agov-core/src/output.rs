use std::io::{self, Write as _};

// ── coord/plan validate output (cli.ex format) ────────────────────────────────
//
// OK  severity: blocking {path}          → stdout
// WARN severity: advisory {path}: {msg} → stdout
// ERR severity: blocking {path}: {msg}  → stderr
// ERR severity: blocking {path}:        → stderr
//     {json_ptr}: {msg}                 → stderr (one line per schema error)

pub fn coord_ok(path: &str) {
    println!("OK  severity: blocking {path}");
}

pub fn coord_warn(path: &str, msg: &str) {
    println!("WARN severity: advisory {path}: {msg}");
}

pub fn coord_err(path: &str, msg: &str) {
    let _ = writeln!(io::stderr(), "ERR severity: blocking {path}: {msg}");
}

pub fn coord_err_schema(path: &str, errors: &[(String, String)]) {
    let _ = writeln!(io::stderr(), "ERR severity: blocking {path}:");
    for (msg, ptr) in errors {
        let _ = writeln!(io::stderr(), "    {ptr}: {msg}");
    }
}

// ── validate sub-command output (ValidateReporter format) ─────────────────────
//
// OK {path_or_label}                   → stdout
// WARN {label}: {location}: {message} → stdout
// ERR {label}: {location}: {message}  → stderr
// ERR {label}                         → stderr

pub fn validate_ok(label: &str) {
    println!("OK {label}");
}

pub fn validate_warn(label: &str, location: &str, message: &str) {
    println!("WARN {label}: {location}: {message}");
}

pub fn validate_err(label: &str, location: &str, message: &str) {
    let _ = writeln!(io::stderr(), "ERR {label}: {location}: {message}");
}

pub fn validate_err_label(label: &str) {
    let _ = writeln!(io::stderr(), "ERR {label}");
}
