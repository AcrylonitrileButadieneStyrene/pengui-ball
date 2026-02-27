use std::sync::Arc;

use leptos::prelude::*;

#[derive(serde::Deserialize)]
pub struct User {
    pub uuid: Arc<str>,
    pub registered: bool,
    pub name: Arc<str>,
    pub rank: u32,
    pub badge: String,
    #[serde(rename = "badgeSlotRows")]
    pub badge_slot_rows: u32,
    #[serde(rename = "badgeSlotCols")]
    pub badge_slot_cols: u32,
    #[serde(rename = "screenshotLimit")]
    pub screenshot_limit: u32,
    pub medals: [u32; 5],
    #[serde(rename = "locationIds")]
    pub location_ids: Option<Vec<u32>>,
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("failed to get player data")]
    BadUser,
    #[error("request error: {0}")]
    Gloo(#[from] gloo_net::Error),
}

pub fn resource() -> LocalResource<Result<User, UserError>> {
    let user = LocalResource::new(|| async {
        let response = gloo_net::http::Request::get("api/info").send().await?;
        if response.status() == 400 {
            return Err(UserError::BadUser);
        }
        Ok(response.json().await?)
    });

    Effect::new(move || {
        if matches!(user.read().as_ref(), Some(Err(UserError::BadUser))) {
            leptos::task::spawn_local_scoped(async move {
                let response = gloo_net::http::Request::get("/api/seiko/logout")
                    .credentials(leptos::web_sys::RequestCredentials::Include)
                    .send()
                    .await
                    .unwrap();
                if response.status() == 200 {
                    user.refetch();
                    let state = crate::state();
                    if state.modal.get_untracked() == Some(crate::modals::Modals::Cors) {
                        state.modal.set(None);
                    }
                }
            });
        }
    });

    user
}
