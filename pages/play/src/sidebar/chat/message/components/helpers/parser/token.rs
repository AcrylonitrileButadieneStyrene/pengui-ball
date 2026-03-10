#[derive(Debug, logos::Logos)]
pub enum Token {
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

    #[regex(r":([a-zA-Z0-9_\-])+(?::|$)", |lex| str_trim(lex, 1, 1))]
    Emoji(String),

    #[regex(r"\[(t?[a-z0-9]{16}(:(\d+))?)\]", |lex| str_trim(lex, 1, 1))]
    Screenshot(String),

    #[token("\\", str)]
    #[token("_", str)]
    #[token("|", str)]
    #[token("~", str)]
    #[token(":", str)]
    #[regex(r"[^\\\*_~|\:]+", str)]
    Text(String),
}

fn str(lex: &mut logos::Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn str_trim(lex: &mut logos::Lexer<'_, Token>, start: usize, end: usize) -> String {
    let slice = lex.slice();
    slice[start..slice.len() - end].to_string()
}
