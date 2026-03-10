use logos::Logos;

mod options;
mod token;

pub use options::Options;
use token::Token;

pub fn parse(text: &str, options: Options) -> String {
    let mut tokens = dbg!(Token::lexer(text))
        .map(|token| match token {
            Ok(Token::Text(text) | Token::Escaped(text)) => Ok(Token::Text(
                text.replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;"),
            )),
            x => {
                leptos::logging::log!("{x:?}");
                x
            }
        })
        .try_collect::<Vec<_>>()
        .unwrap();

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

    tokens
        .into_iter()
        .map(|token| match token {
            Token::Escaped(_) => unreachable!(),
            Token::Italic => "*".to_string(),
            Token::Bold => "**".to_string(),
            Token::Underline => "__".to_string(),
            Token::Spoiler => "||".to_string(),
            Token::Strike => "~~".to_string(),
            // todo: lookup table
            Token::Emoji(id) => format!("<img src=\"/yno/2kki/images/ynomoji/{id}.png\">"),
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
        super::parse(text, super::Options::default())
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
