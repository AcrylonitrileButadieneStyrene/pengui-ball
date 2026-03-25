use std::{collections::HashMap, sync::Arc};

pub struct Options<'a> {
    pub screenshots: Option<&'a str>,
    pub emojis: &'a HashMap<Arc<str>, Arc<str>>,
}
