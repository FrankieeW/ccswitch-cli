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
        .join("debug")
        .join("ccswitch-cli")
}

fn test_db() -> PathBuf {
    project_root().join("test-fixtures").join("test.db")
}

fn run_cli(args: &[&str]) -> (String, String, i32) {
    let bin = test_bin();
    if !bin.exists() {
        panic!("Binary not found at {:?}. Run 'cargo build' first.", bin);
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
        stdout.contains("provider"),
        "Should have provider tags: {}",
        stdout
    );
}

#[test]
fn test_empty_app_type() {
    let (_stdout, _stderr, _code) = run_cli(&["list", "nonexistent"]);
}

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
