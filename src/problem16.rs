// Problem 16. Count Items Matching a Rule
fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> Result<i32, String> {
    let idx = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return Err(format!("Invalid rule key: {rule_key}")),
    };

    i32::try_from(items.iter().filter(|x| x[idx] == rule_value).count())
        .map_err(|_| "Count exceeds i32 range".to_string())
}

pub fn solve() {
    let items = [
        vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
        vec![
            "computer".to_string(),
            "silver".to_string(),
            "lenovo".to_string(),
        ],
        vec![
            "phone".to_string(),
            "gold".to_string(),
            "iphone".to_string(),
        ],
    ];

    let rule_key = "color";
    let rule_value = "silver";
    match count_matches(&items, rule_key, rule_value) {
        Ok(result) => println!("Ok Case - Number of items matching the rule: {result}"),
        Err(e) => println!("Error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matches_ok() {
        let items = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        let rule_key = "color";
        let rule_value = "silver";
        assert_eq!(count_matches(&items, rule_key, rule_value), Ok(1));
    }

    #[test]
    fn test_count_matches_invalid_key() {
        let items = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        let rule_key = "brand";
        let rule_value = "silver";
        assert_eq!(
            count_matches(&items, rule_key, rule_value),
            Err("Invalid rule key: brand".to_string())
        );
    }
}
