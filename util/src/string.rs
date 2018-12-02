/// Calculate the hamming distance between two strings.
pub fn hamming_distance(a: &str, b: &str) -> usize {
    let mut distance = 0;
    for (cha, chb) in a.chars().zip(b.chars()) {
        if cha != chb {
            distance += 1;
        }
    }
    if a.len() != b.len() {
        distance += std::cmp::max(a.len(), b.len()) - std::cmp::min(a.len(), b.len());
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let inputs = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
            String::from("wvxy"),
        ];
        assert_eq!(0, hamming_distance(&inputs[0], &inputs[0]));
        assert_eq!(2, hamming_distance(&inputs[0], &inputs[5]));
        assert_eq!(1, hamming_distance(&inputs[1], &inputs[4]));
        assert_eq!(5, hamming_distance(&inputs[0], &inputs[1]));
        assert_eq!(1, hamming_distance(&inputs[6], &inputs[7]));
        assert_eq!(1, hamming_distance(&inputs[7], &inputs[6]));
    }
}
