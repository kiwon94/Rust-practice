// Problem 25. Count of Matches in Tournament
fn update_matches(remaining_teams: i32, matches: i32) -> i32 {
    if remaining_teams == 1 {
        return matches;
    }
    let new_matches = remaining_teams / 2;
    let new_remaining_teams = (remaining_teams + 1) / 2;
    update_matches(new_remaining_teams, matches + new_matches)
}

fn number_of_matches(n: i32) -> i32 {
    assert!(n > 0, "Number of teams must be positive");
    update_matches(n, 0)
}

pub fn solve() {
    // Solution for Problem 4
    let nums = 7;
    let result = number_of_matches(nums);
    println!("Total number of matches: {result}");
}
