mod domain;

use axum::{extract::Path, http::StatusCode, response::Html};
use domain::models::{Color, State};

pub async fn star() -> Html<&'static str> {
    Html(r#"<div id="star" class="lit"></div>"#)
}

pub async fn present(Path(color): Path<String>) -> Result<Html<String>, StatusCode> {
    let next_color = color
        .parse::<Color>()
        .map_err(|_| StatusCode::IM_A_TEAPOT)?
        .into_next_color();

    let html = format!(
        r#"<div class="present {color}" hx-get="/23/present/{next_color}" hx-swap="outerHTML">
            <div class="ribbon"></div>
            <div class="ribbon"></div>
            <div class="ribbon"></div>
            <div class="ribbon"></div>
        </div>"#
    );

    Ok(Html(html))
}

pub async fn ornament(
    Path((state, n)): Path<(String, String)>,
) -> Result<Html<String>, StatusCode> {
    let next_state = state
        .parse::<State>()
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;

    let html = match next_state {
        State::On => {
            format!(
                r#"<div class="ornament on" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/off/{n}" hx-swap="outerHTML"></div>"#
            )
        }
        State::Off => {
            format!(
                r#"<div class="ornament" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/on/{n}" hx-swap="outerHTML"></div>"#
            )
        }
    };

    Ok(Html(html))
}
