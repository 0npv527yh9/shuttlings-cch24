mod domain;
mod parser;

use axum::{extract::Path, http::StatusCode, response::Html};
use axum_extra::extract::Multipart;
use domain::models::{Color, State};
use itertools::Itertools;
use parser::ParseError;

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

    let n = html_escape::encode_safe(&n);

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

pub async fn lockfile(multipart: Multipart) -> Result<Html<String>, StatusCode> {
    let lock_file = parser::parse_multipart(multipart, "lockfile").await?;
    let packages = parser::parse_lock_file(&lock_file).map_err(Into::<StatusCode>::into)?;
    let checksums = parser::extract_checksums(&packages).map_err(Into::<StatusCode>::into)?;
    let html = checksums.iter().map(into_color_html).join("\n");

    Ok(Html(html))
}

impl From<ParseError> for StatusCode {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Lockfile => StatusCode::BAD_REQUEST,
            ParseError::Checksum => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

#[allow(clippy::ptr_arg)]
fn into_color_html(checksum: &Vec<u8>) -> String {
    let color = hex::encode(&checksum[0..3]);
    let top = checksum[3] as usize;
    let left = checksum[4] as usize;

    format!(r#"<div style="background-color:#{color};top:{top}px;left:{left}px;"></div>"#)
}
