use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Cors>
            <div>Pengui Ball is not officially supported.</div>
            <div>
                To have access to API features, install
                <a
                    href="https://raw.githubusercontent.com/AcrylonitrileButadieneStyrene/pengui-ball/master/js/pengui-ball.user.js"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    this script.
                </a>
            </div>
            <div>
                The API is required for all account-based features, such as badges and leaderboards.
            </div>
        </super::Modal>
    }
}
