use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn compile_shader(filename: &'static str) {
    let curr_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./src/shader/");
    let input = curr_dir.join(filename);
    let mut output = curr_dir.join(filename);
    output.set_extension("spv");

    if !input.exists() {
        todo!();
    }

    let args = [input.to_str().unwrap(), "-o", output.to_str().unwrap()];
    let output = Command::new("glslc").args(&args).output().expect("failed");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    println!("cargo:rerun-if-changed={}", input.to_str().unwrap());
}

fn main() {
    compile_shader("vertex.vert");
    compile_shader("fragment.frag");
}
