use leptos::prelude::*;

#[component]
pub fn Medals(medals: [u8; 5]) -> impl IntoView {
    let [bronze, silver, gold, platinum, diamond] = medals;
    let medals = ((0..diamond).map(|_| "diamond"))
        .chain((0..platinum).map(|_| "platinum"))
        .chain((0..gold).map(|_| "gold"))
        .chain((0..silver).map(|_| "silver"))
        .chain((0..bronze).map(|_| "bronze"))
        .take(5)
        .map(|kind| {
            view! { <img src=format!("_yno/images/medal_{kind}.png") /> }
        })
        .collect::<Vec<_>>();
    view! { <span class=super::style::medals>{medals}</span> }
}
