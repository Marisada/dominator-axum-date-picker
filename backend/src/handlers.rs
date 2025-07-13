use axum::response::Html;

pub async fn greet_handler() -> Html<&'static str> {
    Html("<h1>Nice to meet you!</h1>")
}
