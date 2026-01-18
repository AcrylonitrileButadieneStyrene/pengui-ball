pub struct Player {
    pub name: String,
    pub system: String,
    pub rank: u32,
    pub account: bool,
    pub badge: Option<String>,
    pub medals: [u32; 5],
}
