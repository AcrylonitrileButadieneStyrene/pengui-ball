use leptos::prelude::Set;

pub fn on_message(state: &crate::state::State, parts: &[&str]) {
    match parts {
        ["pc", count] => state.player_count.set(count.parse::<u32>().ok()),
        ["say", uuid, txt] => {}
        ["psay", uuid, txt, msg_id] => {}
        ["gsay", uuid, map, p_map, p_location, x, y, txt, msg_id] => {}
        [cmd, args @ ..] => leptos::logging::warn!("Received command {cmd} with args: {args:?}"),
        [] => leptos::logging::warn!("Received empty command"),
    }
}
