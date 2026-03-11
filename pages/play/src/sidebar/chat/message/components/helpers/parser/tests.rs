use std::collections::HashMap;

fn parse(text: &str) -> String {
    let mut emojis = HashMap::new();
    emojis.insert("test".into(), "test-url".into());
    super::parse(
        text,
        super::Options {
            emojis: &emojis,
            screenshots: None,
        },
    )
}

#[test]
fn xss() {
    assert!(!parse("<script>alert(0);</script>").contains("<script>"))
}

macro_rules! parse_eq {
    ($name: ident, $from: expr, $to: expr) => {
        #[test]
        fn $name() {
            assert_eq!(parse($from), $to)
        }
    };
}

parse_eq!(
    spoiler,
    "|| spoiler||",
    "<span class=\"spoiler\"> spoiler</span>"
);
parse_eq!(italics_1, "* a \\* b *", "<i> a * b </i>");
parse_eq!(italics_2, "\\* a * b *", "* a <i> b </i>");
parse_eq!(
    emoji_small,
    ":test: text",
    r#"<img src="test-url" class="emoji"> text"#
);
parse_eq!(
    emoji_big,
    ":test:",
    r#"<img src="test-url" class="emoji big">"#
);
parse_eq!(
    emoji_large,
    "[:test:]",
    r#"<img src="test-url" class="screenshot">"#
);
