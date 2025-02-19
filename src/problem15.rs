fn sort_sentence(s: &str) -> Result<String, String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.is_empty() {
        return Err("Error: Empty input vector".to_string());
    }

    let words_with_index: Vec<(usize, &str)> = words
        .iter()
        .filter_map(|&word| {
            let (body, index_part) = word.split_at(word.len().saturating_sub(1));
            index_part.parse::<usize>().ok().map(|index| (index, body))
        })
        .collect();

    if words_with_index.len() != words.len() {
        return Err("Error: Some words are missing a valid index".to_string());
    }

    let mut words_with_index = words_with_index;
    words_with_index.sort_unstable_by_key(|&(index, _)| index);

    Ok(words_with_index
        .iter()
        .map(|&(_, word)| word)
        .collect::<Vec<&str>>()
        .join(" "))
}

pub fn solve() {
    let s1: &str = "is2 sentence4 This1 a3";
    match sort_sentence(s1) {
        Ok(result) => println!("The sorted sentence is : {result}"),
        Err(e) => println!("{e}"),
    }
}
