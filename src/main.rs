extern "C" {
    fn add_numbers(a: u64, b: u64) -> u64;
}

fn main() {
    let result = unsafe { add_numbers(5, 7) };
    println!("Result: {}", result);
}

