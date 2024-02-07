extern crate self as htmx_components; // Allows components crate to import from itself when expanding macros.
#[doc(inline)]
pub use crate::server::html_layout::HtmlLayout;
pub mod server;

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
