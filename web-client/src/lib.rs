extern crate self as web_client; // Allows web_client crate to import from itself when expanding macros.
#[doc(inline)]
pub use crate::server::html_layout::HtmlLayout;
use axum::Router;
use tower_http::services::ServeDir;

pub mod server;

pub fn routes() -> Router {
    Router::new().nest_service("/", ServeDir::new("web-client/out"))
}

pub fn concat_attribute(field_value: &str, attribute_value: Option<&String>) -> String {
    let mut values = vec![];

    if !field_value.is_empty() {
        values.push(field_value.trim());
    }

    if let Some(value) = attribute_value {
        values.push(value.trim());
    }

    values.join(" ")
}
