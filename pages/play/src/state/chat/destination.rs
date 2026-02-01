#[derive(Clone, Copy, Default, strum::EnumProperty, strum::FromRepr, strum::VariantArray)]
#[repr(u8)]
pub enum MessageDestination {
    #[strum(props(Name = "Map"))] // these will be used for localization later
    Map,
    #[strum(props(Name = "Party"))]
    Party,
    #[default]
    #[strum(props(Name = "Global"))]
    Global,
}
