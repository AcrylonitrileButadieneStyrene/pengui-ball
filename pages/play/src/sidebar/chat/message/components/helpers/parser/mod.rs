use std::sync::Arc;

use logos::Logos;

mod options;
mod token;

pub use options::Options;
use token::Token;

pub fn parse(text: &str, options: Options) -> String {
    let mut tokens = Token::lexer(text)
        .filter_map(|x| x.ok())
        .map(|token| match token {
            Token::Text(text) | Token::Escaped(text) => Token::Text(
                text.replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;"),
            ),
            Token::Emoji(emoji) if let Some(url) = options.emojis.get(&Arc::from(&*emoji)) => {
                Token::Emoji(url.to_string())
            }
            Token::Emoji(emoji) => Token::Text(format!(":{emoji}:")),
            x => x,
        })
        .collect::<Vec<_>>();

    let only_emojis = tokens.iter().all(|token| matches!(token, Token::Emoji(_)));
    if !only_emojis {
        let mut bold = None;
        let mut italic = None;
        let mut underline = None;
        let mut strike = None;
        let mut spoiler = None;

        for i in 0..tokens.len() {
            let (store, start, end) = match tokens[i] {
                Token::Bold => (&mut bold, "<b>", "</b>"),
                Token::Italic => (&mut italic, "<i>", "</i>"),
                Token::Underline => (&mut underline, "<u>", "</u>"),
                Token::Strike => (&mut strike, "<s>", "</s>"),
                Token::Spoiler => (&mut spoiler, "<span class=\"spoiler\">", "</span>"),
                _ => continue,
            };

            if let Some(previous) = *store {
                tokens[previous] = Token::Text(start.to_string());
                tokens[i] = Token::Text(end.to_string());
                *store = None;
            } else {
                *store = Some(i);
            }
        }
    }

    tokens
        .into_iter()
        .map(|token| match token {
            Token::Escaped(_) => unreachable!(),
            Token::Italic => "*".to_string(),
            Token::Bold => "**".to_string(),
            Token::Underline => "__".to_string(),
            Token::Spoiler => "||".to_string(),
            Token::Strike => "~~".to_string(),
            Token::Emoji(url) => {
                if only_emojis {
                    format!(r#"<img src="{url}" class="emoji big">"#)
                } else {
                    format!(r#"<img src="{url}" class="emoji">"#)
                }
            },
            Token::Screenshot(id) if let Some(ref author) = options.screenshots => {
                // todo: options and temporary
                format!("<img src=\"https://connect.ynoproject.net/2kki/screenshots/{author}/{id}.png\">")
            }
            Token::Screenshot(text) => format!("[{text}]"),
            Token::Text(text) => text,
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    fn parse(text: &str) -> String {
        super::parse(
            text,
            super::Options {
                emojis: &std::collections::HashMap::default(),
                screenshots: None,
            },
        )
    }

    #[test]
    fn xss() {
        assert!(!parse("<script>alert(0);</script>").contains("<script>"))
    }

    #[test]
    fn spoiler() {
        assert_eq!(
            parse("|| spoiler||"),
            "<span class=\"spoiler\"> spoiler</span>"
        );
    }

    #[test]
    fn italics_1() {
        assert_eq!(parse("* a \\* b *"), "<i> a * b </i>");
    }

    #[test]
    fn italics_2() {
        assert_eq!(parse("\\* a * b *"), "* a <i> b </i>");
    }
}
