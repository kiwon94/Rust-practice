// Problem 14. Maximum Number of Words Found in Sentences
fn most_words_found(sentences: &[String]) -> i32 {
    sentences
        .iter()
        .map(|x| {
            i32::try_from(x.split_whitespace().count()).expect("Failed to convert count to i32")
        })
        .max()
        .expect("There must be at lest one sentene")
}

pub fn solve() {
    let sentences: Vec<String> = vec![
        "alice and bob love leetcode".to_string(),
        "i think so too".to_string(),
        "this is great thanks very much".to_string(),
    ];
    let result = most_words_found(&sentences);
    println!("Max number of words found: {result}");
}
