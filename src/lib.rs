
pub fn wc_line(s: &String) -> usize {
    let split = s.lines();

    let len = split.count();

    println!("Count by lines: {}", len);
    len
}

pub fn wc_word(s: &String) -> usize {
    let split = s.split_whitespace();

    let len = split.count();

    println!("Count by words: {}", len);
    len
}

pub fn wc_char(s: &String) -> usize {
    let len = s.chars().count();

    println!("Count by chars: {}", len);
    len
}

pub fn wc_byte(s: &String) -> usize {
    let len = s.len();

    println!("Count by bytes: {}", len);
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wc_word() {
        let hi = String::from("hi there");
        assert_eq!(wc_word(&hi), 2);
    }

    #[test]
    fn test_wc_char() {
        let hi = String::from("hi");
        assert_eq!(wc_char(&hi), 2);
    }

    #[test]
    fn test_wc_byte() {
        let hi = String::from("hi");
        assert_eq!(wc_byte(&hi), 2);
    }

    #[test]
    fn test_wc_line() {
        let hi = String::from("hi");
        assert_eq!(wc_line(&hi), 1);
    }
}
