use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref SINGLE_DIGITS: HashMap<&'static str, i32> = HashMap::from(
      [
          ("One",   1),
          ("Two",   2),
          ("Three", 3),
          ("Four",  4),
          ("Five",  5),
          ("Six",   6),
          ("Seven", 7),
          ("Eight", 8),
          ("Nine",  9),
      ]
    );

    static ref ELEVEN_TO_NINETEEN: HashMap<&'static str, i32> = HashMap::from([
        ("Eleven",      11),
        ("Twelve",      12),
        ("Thirteen",    13),
        ("Fourteen",    14),
        ("Fifteen",     15),
        ("Sixteen",     16),
        ("Seventeen",   17),
        ("Eighteen",    18),
        ("Nineteen",    19),
        ]
    );

    static ref TEN_TO_NINETY: HashMap<&'static str, i32> = HashMap::from([
         ("Ten",        10),
         ("Twenty",     20),
         ("Thirty",     30),
         ("Forty",      40),
         ("Fifty",      50),
         ("Sixty",      60),
         ("Seventy",    70),
         ("Eighty",     80),
         ("Ninety",     90),
    ]);

    static ref POWERS_OF_TEN: HashMap<&'static str, i32> = HashMap::from([
        ("Hundred",     100       ),
        ("Thousand",    1000      ),
        ("Lakh",        100_000   ),
        ("Crore",       10_000_000),
    ]);
}

pub fn word_to_number(word: String) -> Result<i32, String> {

    let res = word.split(" ").fold((0,0), |(acc, sub_acc), val|{
        let mut res = 0;
        if SINGLE_DIGITS.contains_key(val) {
            res = *SINGLE_DIGITS.get(val).unwrap();
            (acc, sub_acc + res)
        }
        else if ELEVEN_TO_NINETEEN.contains_key(val) {
            res = *ELEVEN_TO_NINETEEN.get(val).unwrap();
            (acc, sub_acc + res)
        } else if TEN_TO_NINETY.contains_key(val) {
            res = *TEN_TO_NINETY.get(val).unwrap();
            (acc, sub_acc + res)
        } else if POWERS_OF_TEN.contains_key(val) {
            res = *POWERS_OF_TEN.get(val).unwrap();
            (acc + sub_acc * res, 0)
        } else {
            (acc, sub_acc)
        }
    });

    Ok(res.0 + res.1)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::word_to_number;

    #[rstest]
    #[case("Five", 5)]
    #[case("Ninety Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine", 999_999_9)]
    #[case("Fifty One Lakh Forty Eight Thousand Six Hundred Forty Nine", 51_48_649)]
    #[case("One Lakh Thirty Four Thousand Five Hundred Ninety Seven", 134597)]
    #[case("Ten Thousand One", 10001)]
    #[case("Two Thousand Nineteen", 2019)]
    #[case("Nine Hundred Ninety Nine", 999)]
    fn test_word_to_number(#[case] num: String,
                           #[case] expected: i32) {

        let actual = word_to_number(num);
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_dummy() {
        let actual = word_to_number("Ninety Nine Lakh Ninety Nine Thousand Nine Hundred Ninety Nine".to_string());
        assert_eq!(actual, Ok(99_999_99));
    }
}