use std::process::Command;

const TAILWIND_CSS: &str = "tailwind.css";

fn main() {
    println!("cargo:rerun-if-changed=src/views/");
    println!("cargo:rerun-if-changed={TAILWIND_CSS}");

    let output = Command::new("npx")
        .args([
            "@tailwindcss/cli",
            "-i",
            TAILWIND_CSS,
            "-o",
            "static/styles.css",
            "--minify",
        ])
        .output()
        .expect("failed to execute `tailwindcss`");

    if !output.status.success() {
        panic!(
            "failed to execute `tailwindcss`:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
