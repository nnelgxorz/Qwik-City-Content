#![allow(dead_code)]
use std::{
    borrow::Cow,
    io::Write,
    ops::Range,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ContentRanges {
    pub frontmatter: Range<usize>,
    pub body: Range<usize>,
}

#[inline]
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

#[inline]
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

#[inline]
pub fn write_camel_case<W: Write>(string: &str, w: &mut W) -> std::io::Result<()> {
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
        let _ = w.write(&[push as u8])?;
        uppercase = false;
    }
    Ok(())
}

#[inline]
pub fn write_snake_case<W: Write>(string: &str, w: &mut W) -> std::io::Result<()> {
    for char in string.chars() {
        if char.is_whitespace() || char.is_ascii_punctuation() {
            let _ = w.write(&[b'_'])?;
            continue;
        }
        let _ = w.write(&[char.to_ascii_lowercase() as u8])?;
    }
    Ok(())
}
#[inline]
pub fn output_path<P: AsRef<Path>>(outdir: P, path: &str) -> PathBuf {
    let extension = if path.ends_with(".mdx") {
        ".tsx"
    } else {
        ".ts"
    };
    let mut filename = path.trim_start_matches('/').replace('/', "_");
    filename.push_str(extension);
    return outdir.as_ref().join(&filename);
}
#[inline]
pub fn write_output_path<P: AsRef<Path>, W: Write>(
    outdir: P,
    path: &str,
    w: &mut W,
) -> std::io::Result<()> {
    // let extension = if path.ends_with(".mdx") {
    //     ".tsx"
    // } else {
    //     ".ts"
    // };
    w.write_fmt(format_args!("{}", outdir.as_ref().display()))?;
    for c in path.trim_start_matches('/').chars() {
        if c == '/' {
            w.write_all(&[b'_'])?;
            continue;
        }
        w.write_all(&[c as u8])?;
    }
    Ok(())
    // w.write_all(extension.as_bytes())
}
#[inline]
pub fn html_tag(string: &str) -> &str {
    let start = string.find(|c| c != '<').unwrap_or(0);
    let end = string[start..]
        .find(|c| matches!(c, '/' | '>' | ' '))
        .map(|c| c + 1)
        .unwrap_or(string.len());
    string[start..end].trim()
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, path::Path};

    use super::{capitalize, get_content_ranges, html_tag, output_path};

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
    #[test]
    fn generates_md_output_path() {
        let outdir = "content/generated";
        let filename = "posts/post-1.md";
        let result = output_path(outdir, filename);
        assert_eq!(
            result,
            Path::new("content/generated/posts_post-1.md.ts").to_path_buf()
        )
    }
    #[test]
    fn generates_mdx_output_path() {
        let outdir = "content/generated";
        let filename = "posts/post-1.mdx";
        let result = output_path(outdir, filename);
        assert_eq!(
            result,
            Path::new("content/generated/posts_post-1.mdx.tsx").to_path_buf()
        )
    }
    #[test]
    fn gets_html_tag_name() {
        let tags = [
            "<tag>",
            "<tag >",
            "<tag id=\"id\" class=\"classes\">",
            "<tag id=\"id\" class=\"classes\"/>",
            "<tag id=\"id\" class=\"classes\" >",
            "<tag id=\"id\" class=\"classes\" />",
            "<tag/>",
            "<tag />",
        ];
        for tag in tags {
            assert_eq!(html_tag(tag), "tag")
        }
    }
}
