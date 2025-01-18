fn main() {
    cc::Build::new()
        .file("asm/add_numbers.s") // Path to your assembly file
        .compile("add_numbers");  // Name of the output static library
}

