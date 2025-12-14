use super::*;

#[test]
fn test_median_even() {
    let mut number_list: Vec<i16> = vec![-100, 2, 100, 3];
    let median = median(&mut number_list);
    assert_eq!(median, 2.5, "median of number list should be mean of 2 and 3 (2.5)");
}

#[test]
fn test_median_odd() {
        let mut number_list: Vec<i16> = vec![-100, 2, 100];
    let median = median(&mut number_list);
    assert_eq!(median, f64::from(2), "median of number list should be 2");
}

#[test]
fn test_mode_no_repetition() {
    let mut number_list: Vec<i16> = vec![-100, 2, 100, 3];
    let (occurrences_of_the_mode_value, mode_value) = mode(&mut number_list);
    assert_eq!(occurrences_of_the_mode_value, 1);
}

#[test]
fn test_mode() {
        let mut number_list: Vec<i16> = vec![-100, 2, 2, 2, 2, 100, 100, 3, 3];
    let (occurrences_of_the_mode_value, mode_value) = mode(&mut number_list);
    assert_eq!(occurrences_of_the_mode_value, 4);
    assert_eq!(mode_value, vec![2]);
}