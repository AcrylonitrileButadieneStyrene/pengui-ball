#[derive(serde::Deserialize)]
pub struct User {
    pub uuid: String,
    pub registered: bool,
    pub name: String,
    pub rank: u32,
    pub badge: String,
    #[serde(rename = "badgeSlotRows")]
    pub badge_slot_rows: u32,
    #[serde(rename = "badgeSlotCols")]
    pub badge_slot_cols: u32,
    #[serde(rename = "screenshotLimit")]
    pub screenshot_limit: u32,
    pub medals: [u32; 5],
    #[serde(rename = "locationIds")]
    pub location_ids: Option<Vec<u32>>,
}
