// Problem 20 - Decode the Message
use std::char::TryFromCharError;
use std::collections::HashMap;
fn decode_message(key: &str, message: &str) -> Result<String, TryFromCharError> {
    let mut char_map: HashMap<char, char> = HashMap::new();
    let mut next_char = 'a';

    for ch in key.chars() {
        if ch != ' ' && !char_map.contains_key(&ch) {
            char_map.insert(ch, next_char);
            next_char = char::from(u8::try_from(next_char)? + 1);
        }
    }

    Ok(message
        .chars()
        .map(|ch| *char_map.get(&ch).unwrap_or(&' '))
        .collect())
}

pub fn solve() {
    let key = "the quick brown fox jumps over the lazy dog";
    let message = "vkbs bs t suepuv";
    match decode_message(key, message) {
        Ok(result) => {
            println!("Decoded result: {result}");
        }

        Err(e) => {
            println!("Error: {e}");
        }
    }
}
