// Problem 26. Minimum Number of Moves to Seat Everyone
fn min_moves_to_seat(seats: &[i32], students: &[i32]) -> i32 {
    assert_eq!(
        seats.len(),
        students.len(),
        "Seats and students must have the same length"
    );
    let mut seats = seats.to_vec();
    let mut students = students.to_vec();
    seats.sort_unstable();
    students.sort_unstable();
    seats
        .iter()
        .zip(students.iter())
        .map(|(&seat, &student)| (seat - student).abs())
        .sum()
}

pub fn solve() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    let result = min_moves_to_seat(&seats, &students);
    println!("Total number of moves: {result}");
}
