pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn xor_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    for (a, b) in str1.chars().zip(str2.chars()) {
        let xor_char = (a as u8 ^ b as u8) as char;
        result.push(xor_char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
