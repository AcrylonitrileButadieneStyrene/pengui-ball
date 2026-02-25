use std::sync::Arc;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Expeds {
    pub locations: Vec<ExpedLocation>,
    pub vms: Vec<ExpedVM>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ExpedLocation {
    pub id: u32,
    pub r#type: ExpedLocationType,
    pub game: Arc<str>,
    pub title: Arc<str>,
    pub depth: u8,
    #[serde(rename = "exp")]
    pub experience: u8,
    #[serde(rename = "endDate")]
    pub ends_at: chrono::DateTime<chrono::Local>,
    pub complete: bool,
}

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(from = "i8")]
pub enum ExpedLocationType {
    Free = -1,
    Daily = 0,
    Weekly = 1,
    Weekend = 2,
    Special = 3,
}

#[allow(clippy::fallible_impl_from)]
impl From<i8> for ExpedLocationType {
    fn from(value: i8) -> Self {
        match value {
            -1 => Self::Free,
            0 => Self::Daily,
            1 => Self::Weekly,
            2 => Self::Weekend,
            3 => Self::Special,
            x => panic!("unknown exped type {x}"),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ExpedVM {
    pub id: u32,
    pub game: Arc<str>,
    #[serde(rename = "exp")]
    pub experience: u8,
    #[serde(rename = "endDate")]
    pub ends_at: String,
    pub complete: bool,
}
