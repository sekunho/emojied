use unicode_segmentation::UnicodeSegmentation;

// TODO: Check if \u{200D} succeeds a valid emoji

/// Checks if a string is considered as an emoji
/// There's a limitation, however. This only naively checks if a character is
/// a zero-width joiner or an actual emoji character.
pub fn is_valid(identifier: &str) -> bool {
    identifier.graphemes(true).into_iter().all(|w| {
        w.chars().all(|c| {
            // Have to check if it's numeric since there's a bug in the
            // `unic_emoji_char` crate.
            // https://github.com/open-i18n/rust-unic/issues/280
            (!c.is_numeric())
                // \u{200D} is the zero width joiner
                // https://www.fileformat.info/info/unicode/char/200d/index.htm
                && (unic_emoji_char::is_emoji(c) || c == '\u{200D}' || c == '\u{FE0F}')
        })
    })
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    // https://emojipedia.org/emoji-zwj-sequence/

    #[test]
    fn single_char_emoji() {
        assert!(is_valid("🍆"));
    }

    #[test]
    fn emoji_with_zwjs() {
        assert!(is_valid("👨‍👨‍👧‍👧"))
    }

    #[test]
    fn not_an_emoji() {
        assert!(!is_valid("नमस्ते्"));
    }

    #[test]
    fn digits_should_fail() {
        for num_str in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] {
            assert!(!is_valid(num_str));
        }
    }

    #[test]
    fn emoji_with_variation_selector() {
        // This one has a \u{FE0F} character but it doesn't show in neovim.
        assert!(is_valid("❤️‍🔥"));
    }
}
