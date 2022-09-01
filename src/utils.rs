#![allow(dead_code)]
use std::{borrow::Cow, ops::Range};

#[derive(Debug, PartialEq, Eq)]
pub struct ContentRanges {
    pub frontmatter: Range<usize>,
    pub body: Range<usize>,
}

pub fn get_content_ranges(content: &[u8]) -> ContentRanges {
    if !content.starts_with(b"---") {
        return ContentRanges {
            frontmatter: 0..0,
            body: 0..content.len(),
        };
    }
    let fm_start = 3;
    let mut fm_end = fm_start;
    for bytes in content[fm_start..].windows(3) {
        if bytes == b"---" {
            break;
        }
        fm_end += 1;
    }
    let body_start = if fm_end + 3 > content.len() {
        content.len()
    } else {
        fm_end + 3
    };
    ContentRanges {
        frontmatter: fm_start..fm_end,
        body: body_start..content.len(),
    }
}

pub fn capitalize(string: &str) -> Cow<'_, str> {
    if let Some(first) = string.chars().next() {
        if first.is_ascii_uppercase() {
            return string.into();
        }
        let mut capitalized = String::new();
        capitalized.push(first.to_ascii_uppercase());
        capitalized.push_str(&string[1..]);
        return capitalized.into();
    }
    string.into()
}
pub fn camel_case(string: &str) -> String {
    let mut camel_cased = String::new();
    let mut uppercase = true;
    for char in string.chars() {
        if char.is_whitespace() || char.is_ascii_punctuation() {
            uppercase = true;
            continue;
        }
        let push = if uppercase {
            char.to_ascii_uppercase()
        } else {
            char
        };
        camel_cased.push(push);
        uppercase = false;
    }
    camel_cased
}
pub fn snake_case(string: &str) -> String {
    let mut snake_cased = String::new();
    for char in string.chars() {
        if char.is_whitespace() || char.is_ascii_punctuation() {
            snake_cased.push('_');
            continue;
        }
        snake_cased.push(char.to_ascii_lowercase());
    }
    snake_cased
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::{capitalize, get_content_ranges};

    #[test]
    fn gets_empty_file_ranges() {
        let result = get_content_ranges(b"");
        assert_eq!(result.frontmatter, 0..0);
        assert_eq!(result.body, 0..0)
    }
    #[test]
    fn parses_no_frontmatter() {
        let input = b"No frontmatter here";
        let result = get_content_ranges(input);
        assert_eq!(result.frontmatter, 0..0);
        assert_eq!(result.body, 0..input.len())
    }
    #[test]
    fn parses_only_frontmatter() {
        let frontmatter = "title: Name of File";
        let input = format!("---\n{}\n---", frontmatter);
        let result = get_content_ranges(input.as_bytes());
        assert_eq!(input[result.frontmatter].trim(), frontmatter);
        assert_eq!(input[result.body].trim(), "");
    }
    #[test]
    fn parse_frontmatter_and_content() {
        let frontmatter = "title: Name of File";
        let content = "Content part";
        let input = format!("---\n{}\n---\n{}", frontmatter, content);
        let result = get_content_ranges(input.as_bytes());
        assert_eq!(input[result.frontmatter].trim(), frontmatter);
        assert_eq!(input[result.body].trim(), content);
    }
    #[test]
    fn capitalizes() {
        let input = "lowercase";
        let result = capitalize(input);
        let expected: Cow<str> = "Lowercase".to_owned().into();
        assert_eq!(expected, result)
    }
}