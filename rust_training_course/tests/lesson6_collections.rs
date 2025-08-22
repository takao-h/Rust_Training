use rust_training_course::lesson6_collections::*;
use std::collections::HashMap;

#[test]
fn test_sum_vector() {
    assert_eq!(sum_vector(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(sum_vector(vec![]), 0);
    assert_eq!(sum_vector(vec![-1, 1]), 0);
}

#[test]
fn test_count_word_occurrences() {
    let words = ["apple", "banana", "apple", "orange", "banana", "apple"];
    let mut expected = HashMap::new();
    expected.insert("apple".to_string(), 3);
    expected.insert("banana".to_string(), 2);
    expected.insert("orange".to_string(), 1);

    let result = count_word_occurrences(&words);
    assert_eq!(result, expected);

    let empty_words: &[&str] = &[];
    assert!(count_word_occurrences(empty_words).is_empty());
}
