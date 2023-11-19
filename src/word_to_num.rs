use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref SINGLE_DIGITS: HashMap<i32, &'static str> = HashMap::from(
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

    static ref ELEVEN_TO_NINETEEN: HashMap<i32, &'static str> = HashMap::from([
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

    static ref TEN_TO_NINETY: HashMap<i32, &'static str> = HashMap::from([
         ("Ten",    10),
         ("Twenty", 20),
         ("Thirty", 30),
         ("Forty",  40),
         ("Fifty",  50),
         ("Sixty",  60),
         ("Seventy",70),
         ("Eighty", 80),
         ("Ninety", 90),
    ]);

    static ref POWERS_OF_TEN: HashMap<i32, &'static str> = HashMap::from([
        ("Hundred",  100       ),
        ("Thousand", 1000      ),
        ("Lakh",     100_000   ),
        ("Crore",    10_000_000),
    ]);

}

pub fn word_to_number(word: String) -> Result<i32, String> {
    Err("To be done".to_string())
}