use std::sync::Arc;

pub mod locations;
pub mod players;

pub type Locations = Arc<locations::Locations>;
pub type Players = Arc<players::Players>;
