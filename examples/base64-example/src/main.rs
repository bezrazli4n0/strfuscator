use strfuscator::obfuscate_base64;

fn main() {
    println!("Your magic string: {}", obfuscate_base64!("Hello, World!"));
}
