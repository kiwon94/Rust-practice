// Problem 14. Maximum Number of Words Found in Sentences
fn most_words_found(sentences: &[String]) -> Option<usize> {
    sentences.iter().map(|x| x.split_whitespace().count()).max()
}

pub fn solve() {
    let sentences: Vec<String> = vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ];
    match most_words_found(&sentences) {
        Some(result) => println!("The max number of words found is : {result:?}"),
        None => println!("Error: Empty input"),
    }
}
