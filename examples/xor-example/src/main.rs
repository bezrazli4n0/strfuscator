use strfuscator::obfuscate;

fn main() {
    println!("Your magic string: {}", obfuscate!("Hello, World!", 123));
}
