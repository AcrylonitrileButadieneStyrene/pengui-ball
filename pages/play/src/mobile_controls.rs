use leptos::prelude::*;

stylance::import_style!(pub style, "mobile_controls.module.css");

#[component]
pub fn MobileControls() -> impl IntoView {
    view! {
        <Wrapper>
            <div class=style::left>
                <button style="--k:u" />
                <button style="--k:l" />
                <div style="--k:c" />
                <button style="--k:r" />
                <button style="--k:d" />
            </div>
            <div class=style::right>
                <button style="--k:z" />
                <button style="--k:x" />
                <button style="--k:o" />
                <button style="--k:t" />
                <button style="--k:f" />
                <button style="--k:n" />
            </div>
        </Wrapper>
    }
}

#[island]
fn Wrapper(children: Children) -> impl IntoView {
    view! {
        <div
            class=style::controls
            on:touchdown=get_callback(true)
            on:touchup=get_callback(false)
            on:mousedown=get_callback(true)
            on:mouseup=get_callback(false)
        >
            {children()}
        </div>
    }
}

fn get_callback(is_down: bool) -> impl Fn(leptos::ev::MouseEvent) -> () {
    move |event| {
        if let Some(key) =
            event_target::<leptos::web_sys::HtmlElement>(&event).get_attribute("style")
            && let Some(code) = key.as_bytes().last()
        {
            leptos::logging::log!("clicked {code}. down? {is_down}");
        }
    }
}
