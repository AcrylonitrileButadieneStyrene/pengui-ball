use std::sync::Arc;

use logos::Logos;

#[derive(Debug, logos::Logos)]
enum Token {
    #[regex(r"\\.", |lex| str_trim(lex, 1, 0))]
    Escaped(String),

    #[token("**")]
    Bold,

    #[token("*")]
    Italic,

    #[token("__")]
    Underline,

    #[token("~~")]
    Strike,

    #[token("||")]
    Spoiler,

    #[regex(r":([a-z0-9_\-])+(?::|$)", |lex| str_trim(lex, 1, 1))]
    Emoji(String),

    #[regex(r"\[(t?[a-z0-9]{16}(:(\d+))?)\]", |lex| str_trim(lex, 1, 1))]
    Screenshot(String),

    #[token("_", str)]
    #[token("|", str)]
    #[token("~", str)]
    #[token(":", str)]
    #[regex(r"[^\\\*_~|\:]+", str)]
    Text(String),
}

#[derive(Default)]
pub struct Options {
    pub screenshots: Option<Arc<str>>,
}

fn str(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn str_trim(lex: &mut logos::Lexer<'_, Token>, start: usize, end: usize) -> String {
    let slice = lex.slice();
    slice[start..slice.len() - end].to_string()
}

macro_rules! replace {
    ($tokens: ident, $pattern: pat, $start: expr, $end: expr) => {
        for i in 0..$tokens.len() {
            if matches!($tokens[i], $pattern) {
                let mut matched = None;
                for j in (i + 1)..$tokens.len() {
                    if matches!($tokens[j], $pattern) {
                        matched = Some(j);
                        break;
                    }
                }

                if let Some(j) = matched {
                    $tokens[i] = Token::Text($start.to_string());
                    $tokens[j] = Token::Text($end.to_string())
                } else {
                    break;
                }
            }
        }
    };
}

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
    replace!(tokens, Token::Bold, "<b>", "</b>");
    replace!(tokens, Token::Italic, "<i>", "</i>");
    replace!(tokens, Token::Underline, "<u>", "</u>");
    replace!(tokens, Token::Strike, "<s>", "</s>");
    replace!(
        tokens,
        Token::Spoiler,
        "<span class=\"spoiler\">",
        "</span>"
    );
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
