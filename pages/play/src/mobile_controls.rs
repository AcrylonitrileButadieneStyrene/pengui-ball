use leptos::prelude::*;

stylance::import_style!(pub style, "mobile_controls.module.css");

#[component]
pub fn MobileControls() -> impl IntoView {
    // TODO: dynamically adjust the numeric inputs based off of server config
    view! {
        <Wrapper>
            <div class=style::left>
                <button style="--k:w" />
                <button style="--k:a" />
                <div style:grid-area="c" />
                <button style="--k:s" />
                <button style="--k:d" />
            </div>
            <div class=style::right>
                <button style="--k:z" />
                <button style="--k:x" />
                // 1 one
                <button style="--k:o" />
                // 2 ni
                <button style="--k:n" />
                // 3 three
                <button style="--k:t" />
                // 4 four
                <button style="--k:f" />
                // 5 itsu
                <button style="--k:i" />
                // 6 roku
                <button style="--k:r" />
                // 7 seventh letter of the alphabet
                <button style="--k:g" />
                // 8 eight
                <button style="--k:e" />
                // 9 kyuu
                <button style="--k:k" />
            </div>
        </Wrapper>
    }
}

#[island]
fn Wrapper(children: Children) -> impl IntoView {
    let state = crate::state();
    let send = move |keycode, is_down| {
        state
            .engine
            .send(common::EngineMessage::PressKey(keycode, is_down));
    };

    view! {
        <div
            class=style::controls
            on:touchstart=get_callback(true, send.clone())
            on:touchend=get_callback(false, send.clone())
        >
            {children()}
        </div>
    }
}

fn get_callback(is_down: bool, send: impl Fn(u8, bool)) -> impl Fn(leptos::ev::TouchEvent) -> () {
    move |event| {
        if let Some(key) =
            event_target::<leptos::web_sys::HtmlElement>(&event).get_attribute("style")
            && let Some(code) = key.as_bytes().last()
        {
            send(*code, is_down);
        }
    }
}
