fn main() {
    let result = is_palindrome2("a.".to_string());
    println!("{:?}", result);


    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars()
            .filter(|&char| char.is_alphanumeric())
            .map(|char| char.to_ascii_lowercase())
            .collect();
        if chars.len() < 2 {
            return true;
        }
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left = left + 1;
            right = right - 1;
        }
        true
    }
    // more effient
    pub fn is_palindrome2(s: String) -> bool {
        let str_chars:Vec<char> = s.chars().collect();
        let mut left_point = 0;
        let mut right_point = str_chars.len() - 1;

        while left_point < right_point {
            while !str_chars[left_point].is_ascii_alphanumeric() && left_point < right_point {
                left_point = left_point + 1;
            }

            while !str_chars[right_point].is_ascii_alphanumeric() && left_point < right_point {
                right_point = right_point - 1;
            }

            print!("left:{:?} ...", str_chars[left_point].to_ascii_lowercase());
            println!("right:{:?}", str_chars[right_point].to_ascii_lowercase());

            if str_chars[left_point].to_ascii_lowercase() != str_chars[right_point].to_ascii_lowercase() {
                return false;
            }
            left_point = left_point + 1;

            if right_point >= 1{
                right_point = right_point - 1;
            }else {
                return true
            }
        }
        true
    }
}
