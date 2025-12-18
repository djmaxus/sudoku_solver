use std::process::Command;

#[test]
fn test_main_runs_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that output contains expected sudoku board patterns from board.txt
    assert!(stdout.contains(". . 3"));
    assert!(stdout.contains(". . 7"));

    // Verify we get 9 non-empty lines of output (one per row)
    let non_empty_lines: Vec<_> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(non_empty_lines.len(), 9);

    // Verify first line matches expected format
    let first_line = stdout.lines().next().unwrap();
    assert_eq!(first_line, ". . 3 4 5 6 9 . 8 ");
}
