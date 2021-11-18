use std::collections::HashMap;

fn main() {
    let ans = roman_to_int(String::from("MCMXCIV"));
    println!("{}", ans);
}

pub fn roman_to_int(s: String) -> i32 { 
    // let hashmap: HashMap<char, i32> = HashMap::from([
    //     ('M', 1000), ('D', 500),
    //     ('C', 100), ('L', 50),
    //     ('X', 10), ('V', 5),
    //     ('I', 1)
    // ]);
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    hashmap.insert('M', 1000);
    hashmap.insert('D', 500);
    hashmap.insert('C', 100);
    hashmap.insert('L', 50);
    hashmap.insert('X', 10);
    hashmap.insert('V', 5);
    hashmap.insert('I', 1);
    let length = s.len();
    let last_char = s.chars().nth(length - 1).unwrap();
    let mut result = *hashmap.get(&last_char).unwrap();
    if length == 1 {
        return result;
    }
    for index in (0..=length - 2).rev() {
        let first_char = s.chars().nth(index).unwrap();
        let first_num = *hashmap.get(&first_char).unwrap();
        let second_char = s.chars().nth(index + 1).unwrap();
        let second_num = *hashmap.get(&second_char).unwrap();

        if first_num >= second_num {
            result += first_num;
        } else {
            result -= first_num;
        }
    }
    return result;
}
