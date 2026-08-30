use std::sync::Arc;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeMetadata {
    pub badge_id: Arc<str>,
    pub game: Arc<str>,
    pub group: Arc<str>,
    pub bp: u32,
    pub map_id: u32,
    pub map_x: u32,
    pub map_y: u32,
    pub seconds: u32,
    pub secret: bool,
    pub secret_condition: bool,
    pub hidden: bool,
    pub overlay_type: u32,
    pub art: Box<str>,
    pub animated: bool,
    pub percent: f32,
    pub goals: u32,
    pub goals_total: u32,
    pub tags: Vec<Box<str>>,
    pub unlocked: bool,
    pub new_unlock: bool,
}

impl PartialEq for BadgeMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.badge_id == other.badge_id
    }
}

impl Eq for BadgeMetadata {}

impl PartialOrd for BadgeMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BadgeMetadata {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.map_id.cmp(&other.map_id)
    }
}
