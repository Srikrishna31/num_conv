
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
static ref SINGLE_DIGITS: HashMap<i32, &'static str> = HashMap::from(
  [
      (1, "One"),
      (2, "Two"),
      (3, "Three"),
      (4, "Four"),
      (5, "Five"),
      (6, "Six"),
      (7, "Seven"),
      (8, "Eight"),
      (9, "Nine"),
  ]
);

static ref ELEVEN_TO_NINETEEN: HashMap<i32, &'static str> = HashMap::from([
    (11, "Eleven"),
    (12, "Twelve"),
    (13, "Thirteen"),
    (14, "Fourteen"),
    (15, "Fifteen"),
    (16, "Sixteen"),
    (17, "Seventeen"),
    (18, "Eighteen"),
    (19, "Nineteen"),
    ]
);

static ref TEN_TO_NINETY: HashMap<i32, &'static str> = HashMap::from([
     (10, "Ten"),
     (20, "Twenty"),
     (30, "Thirty"),
     (40, "Forty"),
     (50, "Fifty"),
     (60, "Sixty"),
     (70, "Seventy"),
     (80, "Eighty"),
     (90, "Ninety"),
]);

static ref POWERS_OF_TEN: HashMap<i32, &'static str> = HashMap::from([
    (100, "Hundred"),
    (1000, "Thousand"),
    (100_000, "Lakh"),
    (10_000_000, "Crore"),
]);

static ref POWERS_OF_TEN_LIST: Vec<i32> = vec![
    10_000_000, 100_000, 1000, 100
];
}

const MAX_SUPPORTED_NUM: i32 = 10_000_000;

pub fn number_to_word(mut num: i32) -> Result<String, String> {
    if num > *POWERS_OF_TEN_LIST.first().unwrap() {
        return Err(format!("Only numbers less than {} are supported", POWERS_OF_TEN_LIST.first().unwrap()));
    }

    let mut result = "".to_string();

    //POWERS_OF_TEN_LIST.iter().rev().fold("", )'
    for i in POWERS_OF_TEN_LIST.iter() {
        let deref_i = *i;
        if num >= deref_i {
            let div = num / deref_i;
            let rem = num % deref_i;
            let subres = handle_two_digit_numbers(div);

            result = format!(" {} {} {}", &result, subres, POWERS_OF_TEN.get(i).get_or_insert(&""));

            num = rem;
        }
    }

    result = format!("{} {}", &result, handle_two_digit_numbers(num));

    Ok(String::from(result.trim()))
}

fn handle_two_digit_numbers(num: i32) -> String {
    let mut result = "".to_string();

    if num > 10 && num < 20 {
        result = format!("{}", ELEVEN_TO_NINETEEN.get(&num).get_or_insert(&""));
    } else if num < 10 {
        result = format!("{}", SINGLE_DIGITS.get(&num).get_or_insert(&""));
    } else if num < 100 {
        let tens = (num / 10) * 10;
        let ones = num % 10;
        result = format!("{} {}", TEN_TO_NINETY.get(&tens).get_or_insert(&""), SINGLE_DIGITS.get(&ones).get_or_insert(&""));
    }

    String::from(result.trim())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::number_to_word;

    #[rstest]
    #[case(5, "Five")]
    #[case(999_999_9, "Ninety Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine")]
    #[case(51_48_649, "Fifty One Lakh Forty Eight Thousand Six Hundred Forty Nine")]
    #[case(134597, "One Lakh Thirty Four Thousand Five Hundred Ninety Seven")]
    #[case(10001, "Ten Thousand One")]
    #[case(2019, "Two Thousand Nineteen")]
    #[case(999, "Nine Hundred Ninety Nine")]
    fn test_number_to_word(#[case] num: i32,
    #[case] expected: String) {

        let actual = number_to_word(num);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    #[should_panic(expected = "Only numbers less than 10000000 are supported")]
    fn test_panic_number_to_word() {
        number_to_word(1000_000_5).unwrap();
    }

    #[test]
    fn dummy_test() {
        let actual = number_to_word(2019);
        assert_eq!(actual, Ok("Two Thousand Nineteen".to_string()));
    }
}
