use std::sync::Arc;

use logos::Logos;

mod options;
mod token;

#[cfg(test)]
mod tests;

pub use options::Options;
use token::Token;

pub fn parse(text: &str, options: Options) -> String {
    let mut seen_large_emoji = false;
    let mut tokens = Token::lexer(text)
        .filter_map(Result::ok)
        .map(transform_early(&options, &mut seen_large_emoji))
        .collect::<Vec<_>>();

    let only_emojis = tokens.iter().all(|token| matches!(token, Token::Emoji(_)));
    if !only_emojis {
        transform_spans(&mut tokens);
    }

    tokens
        .into_iter()
        .map(transform_late(only_emojis))
        .collect::<String>()
}

fn transform_early(options: &Options, seen_large_emoji: &mut bool) -> impl FnMut(Token) -> Token {
    move |token| match dbg!(token) {
        Token::Text(text) | Token::Escaped(text) => Token::Text(
            text.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;"),
        ),
        Token::Screenshot(id) if let Some(author) = options.screenshots => {
            // todo: parse temporary and options
            Token::Screenshot(format!(
                "https://connect.ynoproject.net/2kki/screenshots/{author}/{id}.png"
            ))
        }
        Token::Screenshot(text) => Token::Text(format!("[{text}]")),
        Token::Emoji((emoji, large)) if let Some(url) = options.emojis.get(&Arc::from(&*emoji)) => {
            if large {
                if *seen_large_emoji {
                    Token::Text(format!("[{url}]"))
                } else {
                    *seen_large_emoji = true;
                    Token::Screenshot(url.to_string())
                }
            } else {
                Token::Emoji((url.to_string(), false))
            }
        }
        Token::Emoji((emoji, true)) => Token::Text(format!("[:{emoji}:]")),
        Token::Emoji((emoji, false)) => Token::Text(format!(":{emoji}:")),
        x => x,
    }
}

fn transform_spans(tokens: &mut [Token]) {
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

fn transform_late(only_emojis: bool) -> impl Fn(Token) -> String {
    move |token| match token {
        Token::Escaped(_) => unreachable!(),
        Token::Italic => "*".to_string(),
        Token::Bold => "**".to_string(),
        Token::Underline => "__".to_string(),
        Token::Spoiler => "||".to_string(),
        Token::Strike => "~~".to_string(),
        Token::Emoji((url, _)) if only_emojis => format!(r#"<img src="{url}" class="emoji big">"#),
        Token::Emoji((url, _)) => format!(r#"<img src="{url}" class="emoji">"#),
        Token::Screenshot(url) => format!(r#"<img src="{url}" class="screenshot">"#),
        Token::Text(text) => text,
    }
}
