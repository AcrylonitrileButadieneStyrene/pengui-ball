use crate::state::chat::{self, ChatChannel};

#[derive(
    Clone, Copy, Default, PartialEq, Eq, strum::EnumProperty, strum::FromRepr, strum::VariantArray,
)]
#[repr(u8)]
pub enum MessageDestination {
    // the order of these determines the order of the fallback on filter disabled
    #[strum(props(Name = "Map"))] // these will be used for localization later
    Map,
    #[default]
    #[strum(props(Name = "Global"))]
    Global,
    #[strum(props(Name = "Party"))]
    Party,
}

impl MessageDestination {
    pub fn to_channel<'a>(&self, state: &'a chat::State) -> &'a ChatChannel {
        match self {
            MessageDestination::Map => &state.map,
            MessageDestination::Party => &state.party,
            MessageDestination::Global => &state.global,
        }
    }
}
