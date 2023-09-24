use super::Number;

pub(crate) fn get_digits(number: &Number, pos: u32, amount: i32) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();
    let mut index = pos;
    let mut count = 0;
    while count < amount {
        digits.push(number.digits.chars().nth(index as usize).unwrap_or('x'));
        index += 1;
        count += 1;
    }
    digits
}
