use std::process::Command;

fn main() {
    let mold_installed = Command::new("which")
        .arg("mold")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if mold_installed {
        println!("cargo:rustc-link-arg=-fuse-ld=mold");
    }
}