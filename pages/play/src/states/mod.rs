use std::sync::Arc;

pub mod badges;
pub mod interfaces;
pub mod locations;
pub mod players;

pub use interfaces::Interfaces;

pub type Badges = Arc<badges::Badges>;
pub type Locations = Arc<locations::Locations>;
pub type Players = Arc<players::Players>;
