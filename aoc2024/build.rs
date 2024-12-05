// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/day1.S");
    // Use the `cc` crate to compile an assmebly file into a static
    // lib. I guess rustc will just automatically link against it?
    cc::Build::new()
        .file("src/day1.S")
    .compile("aoc");
}
