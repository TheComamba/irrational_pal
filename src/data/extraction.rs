pub(crate) fn get_digits(number: &str, place: i32, amount: i32) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();
    let mut index = place;
    let mut count = 0;
    while count < amount {
        digits.push(number.chars().nth(index as usize).unwrap());
        index += 1;
        count += 1;
    }
    digits
}
