// Problem 20 - Decode the Message
use std::collections::HashMap;
fn decode_message(key: &str, message: &str) -> String {
    let mut char_map: HashMap<char, char> = HashMap::new();
    let mut next_char = 'a';

    for ch in key.chars() {
        if ch != ' ' && !char_map.contains_key(&ch) {
            char_map.insert(ch, next_char);
            next_char = (next_char as u8 + 1) as char;
        }
    }

    message
        .chars()
        .map(|ch| {
            if ch == ' ' {
                ' '
            } else {
                *char_map
                    .get(&ch)
                    .expect("Input should be only lowercase alphabets and whitespaces")
            }
        })
        .collect()
}

pub fn solve() {
    let key = "the quick brown fox jumps over the lazy dog";
    let message = "vkbs bs t suepuv";
    let result = decode_message(key, message);
    println!("Decoded message: {result}");
}
