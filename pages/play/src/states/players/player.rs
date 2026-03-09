use std::sync::Arc;

use reactive_stores::Store;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Store)]
pub struct Player {
    uuid: Option<Arc<str>>,
    name: Option<Arc<str>>,
    badge: Option<Arc<str>>,
    system: Option<Arc<str>>,
    sprite: Option<(Arc<str>, u8)>,
    medals: [u8; 5],
    rank: u8,
    account: bool,
}
