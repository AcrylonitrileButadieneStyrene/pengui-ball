use std::sync::Arc;

pub mod interfaces;
pub mod locations;
pub mod players;

pub use interfaces::Interfaces;
pub type Locations = Arc<locations::Locations>;
pub type Players = Arc<players::Players>;
