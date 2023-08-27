#[cfg(feature = "base64")]
extern crate base64;

use proc_macro::TokenStream;
use quote::quote;

#[cfg(feature = "base64")]
use proc_macro::TokenTree;

#[cfg(feature = "xor")]
use proc_macro2::Literal;

/// Convert input string to `base64` at compile-time.
/// Perform decoding in-place.
/// # Example
/// ```
/// // Remember to add base64 crate as dependency to your project
/// let result = obfuscate_base64!("Hello, World!");
/// ```
#[proc_macro]
#[cfg(feature = "base64")]
pub fn obfuscate_base64(tokens: TokenStream) -> TokenStream {
    use base64::Engine as _;
    // Obtain string literal(with quotes)
    let mut literal_string = String::new();
    for token in tokens {
        let TokenTree::Literal(literal) = token else {
            continue;
        };
        let parsed_literal = literal.to_string();

        literal_string.push_str(&parsed_literal);
    }

    // Exclude quotes from string literal range
    let string_data = &literal_string[1..literal_string.len() - 1];

    // Perform encoding
    let encoded = base64::engine::general_purpose::STANDARD.encode(string_data);

    let result = quote! {
        {
            use base64::Engine as _;
            // This part of code will be used to perform runtime decoding
            String::from_utf8(base64::engine::general_purpose::STANDARD.decode(#encoded).unwrap()).unwrap()
        }
    };

    result.into()
}

/// XOR input string at compile-time.
/// Perform decoding in-place.
/// # Example
/// ```
/// // Remember to add XOR value as second parameter
/// let result = obfuscate_xor!("Hello, World!", 123);
/// ```
#[proc_macro]
#[cfg(feature = "xor")]
pub fn obfuscate_xor(tokens: TokenStream) -> TokenStream {
    let mut tokens_iter = tokens.into_iter();

    // Obtain string literal(with quotes)
    let literal_string = tokens_iter.next().unwrap().to_string();

    // Skip punctuation char
    tokens_iter.next().unwrap();

    // Obtain XOR value from arguments
    let key = tokens_iter
        .next()
        .unwrap()
        .to_string()
        .parse::<u8>()
        .unwrap();

    // Exclude quotes from string literal range
    let string_data = &literal_string[1..literal_string.len() - 1];

    // Apply logical XOR to every character
    let encoded: Vec<u8> = string_data.as_bytes().iter().map(|c| c ^ key).collect();

    // `quote` crate can't work directly with slices and vectors
    // therefore we need convert encoded vector to byte literals
    let encoded = Literal::byte_string(&encoded);

    let result = quote! {
        // This part of code will be used to perform runtime decoding
        String::from_utf8(#encoded.to_vec().iter().map(|&c| c ^ #key).collect()).unwrap()
    };

    result.into()
}
