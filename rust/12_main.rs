
fn main() {
    let ans = int_to_roman(12);
    println!("{}", ans);
}

pub fn int_to_roman(num: i32) -> String {
    let mut given_number = num;
    let vector: Vec<(i32, &str)> = vec!(
    (1, "I"), (4, "IV"),  (5, "V"),
    (9, "IX"), (10, "X"), (40, "XL"),
    (50, "L"), (90, "XC"),  (100, "C"),
    (400, "CD"), (500, "D"), (900, "CM"),
    (1000, "M"),
    );
    let mut reverse_vector = vector;
    reverse_vector.reverse();
    let mut result = "".to_owned();
    for &(value, symbol) in reverse_vector.iter() {
        let division: f64 = f64::from(given_number) / f64::from(value);
        let floor_division = division.floor() as i32;
        if floor_division > 0 {
            let count = floor_division;
            let multiplied_string = symbol.repeat(count as usize);
            result.push_str(&multiplied_string);
            given_number = given_number % value;
        }
    }
    return result;
}
