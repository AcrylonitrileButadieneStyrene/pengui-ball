use std::sync::Arc;

use logos::Logos;

mod options;
mod tokens;

#[cfg(test)]
mod tests;

pub use options::Options;
use tokens::{InputToken, OutputToken, Token};

pub fn parse(text: &str, options: Options) -> String {
    let mut parser = Parser::new();
    let mut tokens = InputToken::lexer(text)
        .filter_map(Result::ok)
        .map(parser.transformer_early(&options))
        .collect::<Vec<_>>();

    let only_emojis = tokens
        .iter()
        .all(|token| matches!(token, Token::Output(OutputToken::Emoji { .. })));
    if !only_emojis {
        transform_spans(&mut tokens);
    }

    tokens
        .into_iter()
        .filter_map(|token| match token {
            Token::Input(_) => None,
            Token::Output(token) => Some(token),
        })
        .map(parser.transformer_late(only_emojis))
        .collect::<String>()
}

struct Parser {
    seen_sticker: bool,
}

impl Parser {
    const fn new() -> Self {
        Self {
            seen_sticker: false,
        }
    }

    fn transformer_early(&mut self, options: &Options) -> impl FnMut(InputToken) -> Token {
        move |tokens| {
            Token::Output(match tokens {
                InputToken::Text(text) | InputToken::Escaped(text) => OutputToken::Text(
                    text.replace('&', "&amp;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;"),
                ),
                InputToken::Screenshot(id) if let Some(author) = options.screenshots => {
                    let (id, temp) = id
                        .strip_prefix('t')
                        .map_or_else(|| (&*id, false), |id| (id, true));

                    // todo: parse options
                    let (id, _options) = if let Some((id, options)) = id.split_once(':') {
                        (id, Some(options))
                    } else {
                        (id, None)
                    };

                    OutputToken::Screenshot {
                        url: format!(
                            "https://ugc.ynoproject.net/screenshots{}/{author}/{id}.png",
                            if temp { "/temp" } else { "" }
                        ),
                    }
                }
                InputToken::Screenshot(text) => OutputToken::Text(format!("[{text}]")),
                InputToken::Emoji((emoji, large))
                    if let Some(url) = options.emojis.get(&Arc::from(&*emoji)) =>
                {
                    if large && std::mem::replace(&mut self.seen_sticker, true) {
                        OutputToken::Text(format!("[{emoji}]"))
                    } else {
                        OutputToken::Emoji {
                            big: large,
                            id: emoji,
                            url: url.clone(),
                        }
                    }
                }
                InputToken::Emoji((emoji, true)) => OutputToken::Text(format!("[:{emoji}:]")),
                InputToken::Emoji((emoji, false)) => OutputToken::Text(format!(":{emoji}:")),
                x => return Token::Input(x),
            })
        }
    }

    #[allow(clippy::unused_self)]
    fn transformer_late(&self, only_emojis: bool) -> impl Fn(OutputToken) -> String {
        const F: bool = false;
        move |tokens| match tokens {
            OutputToken::Emoji { id, url, big: true } => {
                format!(r#"<img src="{url}" title="{id}" class="emoji screenshot">"#)
            }
            OutputToken::Emoji { id, url, big: F } if only_emojis => {
                format!(r#"<img src="{url}" title="{id}" class="emoji big">"#)
            }
            OutputToken::Emoji { id, url, big: F } => {
                format!(r#"<img src="{url}" title="{id}" class="emoji">"#)
            }
            OutputToken::Screenshot { url } => format!(r#"<img src="{url}" class="screenshot">"#),
            OutputToken::Text(text) => text,
        }
    }
}

fn transform_spans(tokens: &mut [Token]) {
    let mut bold = None;
    let mut italic = None;
    let mut underline = None;
    let mut strike = None;
    let mut spoiler = None;

    for i in 0..tokens.len() {
        let Token::Input(token) = &tokens[i] else {
            continue;
        };

        let (store, start, end) = match token {
            InputToken::Bold => (&mut bold, "<b>", "</b>"),
            InputToken::Italic => (&mut italic, "<i>", "</i>"),
            InputToken::Underline => (&mut underline, "<u>", "</u>"),
            InputToken::Strike => (&mut strike, "<s>", "</s>"),
            InputToken::Spoiler => (&mut spoiler, "<span class=\"spoiler\">", "</span>"),
            _ => continue,
        };

        if let Some(previous) = *store {
            tokens[previous] = Token::Output(OutputToken::Text(start.to_string()));
            tokens[i] = Token::Output(OutputToken::Text(end.to_string()));
            *store = None;
        } else {
            *store = Some(i);
        }
    }

    // revert any remaining unmatched markdown pairs to their normal form
    for token in tokens {
        let Token::Input(input) = token else {
            continue;
        };

        *token = Token::Output(match input {
            InputToken::Italic => OutputToken::Text("*".to_string()),
            InputToken::Bold => OutputToken::Text("**".to_string()),
            InputToken::Underline => OutputToken::Text("__".to_string()),
            InputToken::Spoiler => OutputToken::Text("||".to_string()),
            InputToken::Strike => OutputToken::Text("~~".to_string()),
            _ => continue,
        });
    }
}
