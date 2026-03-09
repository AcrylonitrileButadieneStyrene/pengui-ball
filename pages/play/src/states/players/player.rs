use std::sync::Arc;

use reactive_stores::Store;

#[derive(Clone, Default, PartialEq, Eq, Hash, Store)]
pub struct Player {
    name: Option<Arc<str>>,
    badge: Option<Arc<str>>,
    system: Option<Arc<str>>,
    sprite: Option<(Arc<str>, u8)>,
    medals: [u8; 5],
    rank: u8,
    account: bool,
    friend: Option<super::friend::Friend>,
}
