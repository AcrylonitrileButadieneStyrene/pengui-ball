use std::sync::Arc;

use crate::{
    sidebar::chat::message::components::{
        global::GlobalMessage, map::MapMessage, party::PartyMessage,
    },
    state::chat::{self, ChatChannel},
};

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::IntoStaticStr,
    strum::FromRepr,
    strum::VariantArray,
)]
#[repr(u8)]
pub enum MessageDestination {
    // the order of these determines the order of the fallback on filter disabled
    Map,
    #[default]
    Global,
    Party,
}

impl MessageDestination {
    pub fn to_channel(self, state: &chat::State) -> Arc<ChatChannel> {
        match self {
            Self::Map => state.channel::<MapMessage>(),
            Self::Party => state.channel::<PartyMessage>(),
            Self::Global => state.channel::<GlobalMessage>(),
        }
    }
}
