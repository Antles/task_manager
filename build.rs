use std::process::Command;

fn main() {
    // Run SQLx migrations
    let output = Command::new("sqlx")
        .args(&["migrate", "run"])
        .output()
        .expect("Failed to run migrations");

    if !output.status.success() {
        panic!(
            "Migration failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=migrations/");
}
