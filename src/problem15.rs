// Problem 15. Sorting the Sentence
fn sort_sentence(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();

    let mut words_with_index: Vec<(usize, &str)> = words
        .iter()
        .map(|x| {
            let index = usize::try_from(
                x.chars()
                    .last()
                    .expect("Each word has at aleast one character")
                    .to_digit(10)
                    .expect("Failed to convert char to digit"),
            )
            .expect("Failed to convert digit to usize");
            let words_stripped = &x[..x.len() - 1];
            (index, words_stripped)
        })
        .collect();

    words_with_index.sort_unstable_by_key(|&(index, _)| index);
    let result: String = words_with_index
        .iter()
        .map(|&(_, word)| word)
        .collect::<Vec<&str>>()
        .join(" ");
    result
}

pub fn solve() {
    let s: String = "is2 sentence4 This1 a3".to_string();
    let result = sort_sentence(&s);
    println!("Sorted sentence: {result}");
}
