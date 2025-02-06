fn main() {
    // Add Two (Integer & String) Integer
    let num1: i32 = 12;
    let num2: i32 = 5;
    let num3: i32 = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, num3);
    // String
    let mut s1: String = String::from("Hello, ");
    let s2: String = String::from("World!");
    s1.push_str(&s2);
    println!("{}", s1);

    // Concatenation of Array (Integer & String) Integer
    let mut v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = vec![4, 5, 6];
    for i in v2.iter() {
        v1.push(*i);
    }
    println!("{:?}", v1);
    // String Version using Vec<String>
    let mut v1: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
    let v2: Vec<String> = vec!["Rust".to_string(), "Programming".to_string()];
    for s in v2.iter() {
        v1.push(s.clone());
    }
    println!("{:?}", v1);
    // String Version using Vec<&str>
    let mut v1: Vec<&str> = vec!["Hello", "World"];
    let v2: Vec<&str> = vec!["Rust", "Programming"];
    for s in v2.iter() {
        v1.push(s);
    }
    println!("{:?}", v1);

    // Build Array from Permutation (Integer & String) Integer
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
    // String
    let v1: Vec<&str> = vec!["0", "2", "1", "5", "3", "4"];
    let mut v2: Vec<&str> = Vec::new();
    for val in v1.iter() {
        match val.parse::<usize>() {
            Ok(index) => {
                if index < v1.len() {
                    v2.push(v1[index]);
                } else {
                    println!("Invalid index: {}", val);
                }
            }
            Err(_) => {
                println!("Failed to parse index: {}", val);
            }
        }
    }
    println!("{:?}", v2);

    // Running Sum of 1D Array (Integer & String) Integer
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let mut v2: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for val in v1.iter() {
        sum += *val;
        v2.push(sum);
    }
    println!("{:?}", v2);
    // String
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let mut v2: Vec<String> = Vec::new();
    let mut sum: String = String::new();
    for val in chars.iter() {
        sum.push(*val);
        v2.push(sum.clone());
    }
    println!("{:?}", v2);
}
