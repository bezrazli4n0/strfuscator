use strfuscator::obfuscate_xor;

fn main() {
    println!(
        "Your magic string: {}",
        obfuscate_xor!("Hello, World!", 123)
    );
}
