use super::repeat::repeat;
use darth_rust::DarthRust;
use rand::prelude::*;
pub const EMOJIS: [char; 262] = [
    '😀', '😄', '😊', '🙂', '😎', '😍', '🤩', '😂', '🤣', '😉', '😇', '🥰', '😋', '😜', '🤪', '😛',
    '🥳', '😺', '🐶', '🐱', '🐭', '🐰', '🦊', '🐻', '🐼', '🦁', '🐯', '🐮', '🐷', '🐸', '🐵', '🐔',
    '🐧', '🦆', '🦉', '🦄', '🐝', '🐞', '🦋', '🐢', '🐍', '🦎', '🦖', '🦕', '🐙', '🦑', '🦐', '🦞',
    '🦀', '🐳', '🐬', '🐟', '🐠', '🐡', '🦈', '🐋', '🐊', '🐆', '🐅', '🐃', '🐂', '🐄', '🦌', '🐪',
    '🐫', '🦙', '🦘', '🦥', '🦡', '🐘', '🦏', '🦛', '🐐', '🐏', '🐑', '🦒', '🐓', '🦃', '🦆', '🐕',
    '🐩', '🐈', '🐇', '🐁', '🐀', '🦔', '🐾', '🐉', '🐲', '🌵', '🌴', '🌷', '🌸', '🌹', '🌺', '🌻',
    '🌼', '🌽', '🌾', '🌿', '🍀', '🍁', '🍂', '🍃', '🍄', '🍅', '🍆', '🍇', '🍈', '🍉', '🍊', '🍋',
    '🍌', '🍍', '🍎', '🍏', '🍐', '🍑', '🍒', '🍓', '🍔', '🍕', '🍖', '🍗', '🍘', '🍙', '🍚', '🍛',
    '🍜', '🍝', '🍞', '🍟', '🍠', '🍡', '🍢', '🍣', '🍤', '🍥', '🍦', '🍧', '🍨', '🍩', '🍪', '🍫',
    '🍬', '🍭', '🍮', '🍯', '🍰', '🍱', '🍲', '🍳', '🍴', '🍵', '🍶', '🍷', '🍸', '🍹', '🍺', '🍻',
    '🍼', '🍾', '🍿', '🎀', '🎁', '🎂', '🎃', '🎄', '🎅', '🎆', '🎇', '🎈', '🎉', '🎊', '🎋', '🎌',
    '🎍', '🎎', '🎏', '🎐', '🎑', '🎒', '🎓', '🎠', '🎡', '🎢', '🎣', '🎤', '🎥', '🎦', '🎧', '🎨',
    '🎩', '🎪', '🎫', '🎬', '🎭', '🎮', '🎯', '🎰', '🎱', '🎲', '🎳', '🎴', '🎵', '🎶', '🎷', '🎸',
    '🎹', '🎺', '🎻', '🎼', '🎽', '🎾', '🎿', '🏀', '🏁', '🏂', '🏃', '🏄', '🏅', '🏆', '🏇', '🏈',
    '🏉', '🏊', '🏏', '🏐', '🏑', '🏒', '🏓', '🏠', '🏡', '🏢', '🏣', '🏤', '🏥', '🏦', '🏧', '🏨',
    '🏩', '🏪', '🏫', '🏬', '🏭', '🏮', '🏯', '💒', '🔥', '🔦', '🔧', '🔨', '🔩', '🔪', '🔫', '🔬',
    '🔭', '🔮', '🔯', '🔰', '🔱', '🔲',
];
pub const SPECIAL_CHARACTERS: [char; 28] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '[', ']', '{', '}', '|',
    '\\', '/', '<', '>', ',', '.', '?', ':', ';',
];

#[derive(Debug, DarthRust)]
pub struct RandomBytes {
    pub uppercase_range: (u32, u32),
    pub lowercase_range: (u32, u32),
    pub number_range: (u32, u32),
    pub special_range: (u32, u32),
    pub emoji_range: (u32, u32),
}

pub trait RandomBytesTrait {
    fn create(&self) -> String;
}

impl RandomBytesTrait for RandomBytes {
    fn create(&self) -> String {
        let (uppercase_min, uppercase_max) = self.uppercase_range;
        let (lowercase_min, lowercase_max) = self.lowercase_range;
        let (number_min, number_max) = self.number_range;
        let (special_min, special_max) = self.special_range;
        let (emoji_min, emoji_max) = self.emoji_range;
        let breaker = |min: u32, max: u32| thread_rng().gen_range(min + 1..=max + 1);
        let special_characters = SPECIAL_CHARACTERS;
        let emojis = EMOJIS;
        let uppercase_breaker = breaker(uppercase_min, uppercase_max);
        let lowercase_breaker = breaker(lowercase_min, lowercase_max);
        let number_breaker = breaker(number_min, number_max);
        let special_characters_breaker = breaker(special_min, special_max);
        let emoji_breaker = breaker(emoji_min, emoji_max);

        let mut input: String = "".to_string();
        let mut add_random_uppercase_char = 0;
        let mut add_random_lowercase_char = 0;
        let mut add_random_number_char = 0;
        let mut add_random_emoji_char = 0;
        let mut add_special_characters = 0;

        repeat(
            &mut input,
            &mut add_random_uppercase_char,
            uppercase_breaker,
            thread_rng().gen_range('A'..='Z'),
        );
        repeat(
            &mut input,
            &mut add_random_lowercase_char,
            lowercase_breaker,
            thread_rng().gen_range('a'..='z'),
        );
        repeat(
            &mut input,
            &mut add_random_number_char,
            number_breaker,
            thread_rng().gen_range('0'..='9'),
        );
        repeat(
            &mut input,
            &mut add_random_emoji_char,
            emoji_breaker,
            *emojis.choose(&mut thread_rng()).unwrap(),
        );
        repeat(
            &mut input,
            &mut add_special_characters,
            special_characters_breaker,
            *special_characters.choose(&mut thread_rng()).unwrap(),
        );
        let mut chars: Vec<char> = input.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        chars.iter().collect()
    }
}
