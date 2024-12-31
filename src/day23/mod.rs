use axum::response::Html;

pub async fn star() -> Html<&'static str> {
    Html(r#"<div id="star" class="lit"></div>"#)
}
