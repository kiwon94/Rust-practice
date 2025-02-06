// Problem 3. Build Array from Permutation

pub fn solve() {
    // Solution for Problem 3
    let v1: Vec<i32> = vec![0, 2, 1, 5, 3, 4];
    let mut v2: Vec<i32> = Vec::new();
    for val in v1.iter() {
        match *val {
            index if index >= 0 && index < v1.len() as i32 => {
                v2.push(v1[index as usize]);
            }
            _ => {
                println!("Invalid index: {}", val);
            }
        }
    }
    println!("{:?}", v2);

    // Solution for Problem 3 (String ver.)
    // let v1: Vec<&str> = vec!["0", "2", "1", "5", "3", "4"];
    // let mut v2: Vec<&str> = Vec::new();
    // for val in v1.iter() {
    //     match val.parse::<usize>() {
    //         Ok(index) => {
    //             if index < v1.len() {
    //                 v2.push(v1[index]);
    //             } else {
    //                 println!("Invalid index: {}", val);
    //             }
    //         }
    //         Err(_) => {
    //             println!("Failed to parse index: {}", val);
    //         }
    //     }
    // }
    // println!("{:?}", v2);
}
