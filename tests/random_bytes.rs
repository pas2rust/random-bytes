use random_bytes::random_bytes::{RandomBytes, RandomBytesTrait, EMOJIS, SPECIAL_CHARACTERS};

lazy_static::lazy_static! {
    static ref RANDOM: RandomBytes = RandomBytes::new(
        (10, 25),
        (10, 25),
        (10, 25),
        (10, 25),
        (10, 25),
    );
}

#[test]
fn random_contains_letters_uppercase_range_10_25() {
    let uppercase_letters: Vec<char> = ('A'..='Z').collect();
    let uppercase_count = RANDOM.create().chars().filter(|c| uppercase_letters.contains(c)).count();
    RANDOM.print_err();
    assert!(uppercase_count >= 10 && uppercase_count <= 25);
}

#[test]
fn random_contains_letters_lowercase_range_10_25() {
    let lowercase_letters: Vec<char> = ('a'..='z').collect();
    let lowercase_count = RANDOM.create().chars().filter(|c| lowercase_letters.contains(c)).count();
    RANDOM.print_err();
    assert!(lowercase_count >= 10 && lowercase_count <= 25);
}

#[test]
fn random_contains_numbers_range_10_25() {
    let numbers: Vec<char> = ('0'..='9').collect();
    let number_count = RANDOM.create().chars().filter(|c| numbers.contains(c)).count();
    RANDOM.print_err();
    assert!(number_count >= 10 && number_count <= 25);
}

#[test]
fn random_contains_special_chars_range_10_25() {
    let special_chars = SPECIAL_CHARACTERS;
    let special_count = RANDOM.create().chars().filter(|c| special_chars.contains(c)).count();
    RANDOM.print_err();
    assert!(special_count >= 10 && special_count <= 25);
}

#[test]
fn random_contains_emojis_range_10_25() {
    let emojis = EMOJIS;
    let emoji_count = RANDOM.create().chars().filter(|c| emojis.contains(c)).count();
    RANDOM.print_err();
    assert!(emoji_count >= 10 && emoji_count <= 25);
}
