use std::env;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn test_bin() -> PathBuf {
    project_root()
        .join("target")
        .join("release")
        .join("ccswitch-cli")
}

fn test_db() -> PathBuf {
    project_root().join("test-fixtures").join("test.db")
}

fn run_cli(args: &[&str]) -> (String, String, i32) {
    let bin = test_bin();
    if !bin.exists() {
        panic!("Binary not found at {:?}. Run 'cargo build --release' first.", bin);
    }

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("cc-switch.db");
    std::fs::copy(test_db(), &db_path).unwrap();

    let mut cmd = Command::new(&bin);
    cmd.env("CCSWITCH_DB_PATH", &db_path);

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.code().unwrap_or(-1))
}

/// Helper: run CLI with a writable temp DB copy, return the DB path so tests
/// can inspect state after the command.
fn run_cli_writable(args: &[&str]) -> (String, String, i32, TempDir) {
    let bin = test_bin();
    if !bin.exists() {
        panic!("Binary not found at {:?}. Run 'cargo build --release' first.", bin);
    }

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("cc-switch.db");
    std::fs::copy(test_db(), &db_path).unwrap();

    let mut cmd = Command::new(&bin);
    cmd.env("CCSWITCH_DB_PATH", &db_path);

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.code().unwrap_or(-1), temp_dir)
}

// ---------------------------------------------------------------------------
// Core commands
// ---------------------------------------------------------------------------

#[test]
fn test_list_claude_providers() {
    let (stdout, stderr, code) = run_cli(&["list", "claude"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("Providers"),
        "Expected 'Providers' in output: {}",
        stdout
    );
    assert!(
        stdout.contains("Anthropic") || stdout.contains("OpenRouter"),
        "Expected provider names in output: {}",
        stdout
    );
}

#[test]
fn test_list_opencode_providers() {
    let (stdout, stderr, code) = run_cli(&["list", "opencode"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("Providers"),
        "Should show providers: {}",
        stdout
    );
    assert!(
        stdout.contains("MiniMax") || stdout.contains("DeepSeek"),
        "Expected MiniMax or DeepSeek: {}",
        stdout
    );
}

#[test]
fn test_current_provider() {
    let (stdout, stderr, code) = run_cli(&["current", "claude"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("Current"),
        "Should show current: {}",
        stdout
    );
}

#[test]
fn test_health_check() {
    let (stdout, stderr, code) = run_cli(&["health", "claude"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("Health") || stdout.contains("Healthy") || stdout.contains("Unhealthy"),
        "Should show health status: {}",
        stdout
    );
}

#[test]
fn test_switch_with_dry_run() {
    let (stdout, stderr, code) =
        run_cli(&["switch", "claude", "--provider", "openrouter", "--dry-run"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("Would switch") || stdout.contains("dry_run"),
        "Should show dry run output: {}",
        stdout
    );
}

#[test]
fn test_switch_invalid_provider() {
    let (stdout, stderr, code) = run_cli(&["switch", "claude", "--provider", "nonexistent"]);
    assert!(
        code != 0 || stderr.contains("not found") || stderr.contains("Error"),
        "Should error for unknown provider: {} / {}",
        stdout,
        stderr
    );
}

// ---------------------------------------------------------------------------
// App-type validation — invalid app values must be rejected before DB access
// ---------------------------------------------------------------------------

#[test]
fn test_invalid_app_type_rejected() {
    let (stdout, stderr, code) = run_cli(&["list", "nonexistent"]);
    // Exit 2 from Clap because "nonexistent" is not a valid AppType value.
    assert_eq!(code, 2, "Invalid app type should exit with code 2: {} / {}", stdout, stderr);
    assert!(
        stderr.contains("possible values"),
        "Error should list valid options: {}",
        stderr
    );
}

// ---------------------------------------------------------------------------
// AI mode
// ---------------------------------------------------------------------------

#[test]
fn test_ai_mode_output() {
    let (stdout, stderr, code) = run_cli(&["--ai", "list", "claude"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("<ccswitch"),
        "AI mode should output XML: {}",
        stdout
    );
    assert!(
        stdout.contains("</ccswitch>"),
        "AI mode output must be well-formed (close tag): {}",
        stdout
    );
    assert!(
        stdout.contains("<provider"),
        "Should have provider tags: {}",
        stdout
    );
}

/// Verify that current --ai produces well-formed XML (was broken: missing close tag).
#[test]
fn test_ai_current_well_formed() {
    let (stdout, stderr, code) = run_cli(&["--ai", "current", "claude"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("<ccswitch") && stdout.contains("</ccswitch>"),
        "current --ai must be well-formed XML: {}",
        stdout
    );
}

#[test]
fn test_ai_switch_well_formed() {
    let (stdout, stderr, code) = run_cli(&["--ai", "switch", "claude", "--provider", "openrouter"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("<ccswitch") && stdout.contains("</ccswitch>"),
        "switch --ai must be well-formed XML: {}",
        stdout
    );
    assert!(
        stdout.contains("<applied>true</applied>"),
        "Non-dry-run switch should show applied=true: {}",
        stdout
    );
}

// ---------------------------------------------------------------------------
// XML injection / escaping — ensure special chars in DB don't break output
// ---------------------------------------------------------------------------


// ---------------------------------------------------------------------------
// DB mutation — switch actually writes
// ---------------------------------------------------------------------------

#[test]
fn test_switch_actually_updates_db() {
    // Use a writable temp DB so we can inspect the mutation.
    let (_stdout, _stderr, code, temp_dir) =
        run_cli_writable(&["switch", "claude", "--provider", "openrouter"]);
    assert_eq!(code, 0, "Switch should succeed");

    let db_path = temp_dir.path().join("cc-switch.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    let current: String = conn
        .query_row(
            "SELECT id FROM providers WHERE app_type = 'claude' AND is_current = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        current, "openrouter",
        "openrouter should now be current provider"
    );
}

#[test]
fn test_switch_dry_run_does_not_mutate() {
    // Run a dry-run, then verify the DB is unchanged.
    let (stdout, stderr, code, temp_dir) =
        run_cli_writable(&["switch", "claude", "--provider", "openrouter", "--dry-run"]);
    assert_eq!(code, 0, "Dry-run should succeed: {}", stderr);
    assert!(
        stdout.contains("Would switch"),
        "Dry-run should show 'Would switch': {}",
        stdout
    );

    let db_path = temp_dir.path().join("cc-switch.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    let current: String = conn
        .query_row(
            "SELECT id FROM providers WHERE app_type = 'claude' AND is_current = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        current, "anthropic",
        "Dry-run must not mutate DB — anthropic should still be current"
    );
}

// ---------------------------------------------------------------------------
// Meta
// ---------------------------------------------------------------------------

#[test]
fn test_version_flag() {
    let (stdout, stderr, code) = run_cli(&["--version"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("ccswitch-cli") || stdout.contains("0.1"),
        "Should show version: {}",
        stdout
    );
}

#[test]
fn test_help_flag() {
    let (stdout, stderr, code) = run_cli(&["--help"]);
    assert_eq!(code, 0, "stderr: {}", stderr);
    assert!(
        stdout.contains("list") && stdout.contains("switch") && stdout.contains("current"),
        "Should show commands in help: {}",
        stdout
    );
}

#[test]
fn test_completions_generates_output() {
    let (stdout, stderr, code) = run_cli(&["completions", "bash"]);
    assert_eq!(code, 0, "Completions should succeed: {}", stderr);
    assert!(
        stdout.len() > 100,
        "Completions output should be non-trivial: {} bytes",
        stdout.len()
    );
}
