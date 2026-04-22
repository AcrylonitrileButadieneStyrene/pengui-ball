#![allow(non_snake_case)]

use leptos::prelude::*;

#[component]
pub fn BadgeTools() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();

    view! {
        <leptos_meta::Link rel="stylesheet" href="/css/badge_tools.css" />
        <leptos_meta::Title text="Badge Tools" />
        <leptos_meta::Meta name="robots" content="noindex" />

        <div class="center">
            <main>
                <Inner games=config.games.iter().map(|game| game.id.clone()).collect() />
            </main>
        </div>
    }
}

#[island]
fn Inner(games: Vec<std::sync::Arc<str>>) -> impl IntoView {
    let (selected, set_selected) = signal("None".to_string());
    let (map, set_map) = signal(0);
    let (x, set_x) = signal(0);
    let (y, set_y) = signal(0);

    let resolver = locations::Resolver::new();
    let location = move || {
        let resolved = resolver.resolve(&locations::Location {
            game: selected().into(),
            map: map(),
            previous: None,
            x: x() as _,
            y: y() as _,
        });

        format!("{resolved:#?}")
    };

    view! {
        <GameSelector selected set_selected games />
        <NumberInput value=map set_value=set_map />
        <NumberInput value=x set_value=set_x />
        <NumberInput value=y set_value=set_y />
        {location}
    }
}

#[component]
fn GameSelector(
    selected: ReadSignal<String>,
    set_selected: WriteSignal<String>,
    games: Vec<std::sync::Arc<str>>,
) -> impl IntoView {
    let games = games
        .into_iter()
        .map(|game| view! { <option>{game}</option> })
        .collect::<Vec<_>>();

    let on_change = move |event| {
        let value = event_target_value(&event);
        set_selected(value);
    };

    view! {
        <label>
            Game
            <select on:change=on_change prop:value=selected>
                <option selected>None</option>
                {games}
            </select>
        </label>
    }
}

#[component]
fn NumberInput(value: ReadSignal<u16>, set_value: WriteSignal<u16>) -> impl IntoView {
    let on_change = move |event| {
        let Ok(value) = event_target_value(&event).parse() else {
            return;
        };
        set_value(value);
    };

    view! {
        <input type="number" value=value on:change=on_change />
    }
}
