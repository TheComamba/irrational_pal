use super::Number;
use std::cmp::min;

pub(crate) fn get_digits(number: &Number, pos: u32, amount: u32) -> Vec<char> {
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

pub(crate) fn number_representation(number: &Number, pos: u32) -> String {
    let name = number.name;
    let amount = min(pos, 10);
    let start = pos - amount;
    let mut digits = get_digits(number, start, amount);
    if start == 0 && !digits.is_empty() {
        digits.insert(1, '.');
    } else if start > 0 {
        for _ in 0..3 {
            digits.insert(0, '.');
        }
    }
    let digits = digits.iter().collect::<String>();
    format!("{}={}", name, digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::e::E;

    #[test]
    fn test_number_text() {
        assert_eq!(number_representation(&E, 0), "e=");
        assert_eq!(number_representation(&E, 1), "e=2.");
        assert_eq!(number_representation(&E, 2), "e=2.7");
        assert_eq!(number_representation(&E, 3), "e=2.71");
        assert_eq!(number_representation(&E, 4), "e=2.718");
        assert_eq!(number_representation(&E, 5), "e=2.7182");
        assert_eq!(number_representation(&E, 6), "e=2.71828");
        assert_eq!(number_representation(&E, 7), "e=2.718281");
        assert_eq!(number_representation(&E, 8), "e=2.7182818");
        assert_eq!(number_representation(&E, 9), "e=2.71828182");
        assert_eq!(number_representation(&E, 10), "e=2.718281828");
        assert_eq!(number_representation(&E, 11), "e=...7182818284");
    }
}
