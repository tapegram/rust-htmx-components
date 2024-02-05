use super::opt_attrs::opt_attrs;
use rscx::{component, props};
use std::collections::HashMap;
use web_macros::*;

#[html_element]
pub struct HtmlElementProps {
    #[builder(default)]
    children: String,

    #[builder(setter(into), default=String::from("HtmlElement"))]
    component_name: String,

    #[builder(setter(into), default=String::from("div"))]
    tag: String,
}

#[component]
pub fn HtmlElement(props: HtmlElementProps) -> String {
    let attrs = opt_attrs(
        HashMap::from([("data-rsx", props.component_name.clone())])
            .into_iter()
            .chain(props.html_attrs_to_hashmap())
            .collect::<HashMap<&str, String>>(),
    );

    format!(
        "<{} {}>{}</{}>",
        props.tag, attrs, props.children, props.tag
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::attrs::Attrs;
    use rscx::html;

    #[tokio::test]
    async fn test_with_no_attrs() {
        let html = html! {
            <HtmlElement />
        };

        assert_eq!(html, String::from("<div data-rsx=\"HtmlElement\"></div>"));
    }

    #[tokio::test]
    async fn test_with_tag_set() {
        let html = html! {
            <HtmlElement tag="button" />
        };

        assert_eq!(
            html,
            String::from("<button data-rsx=\"HtmlElement\"></button>")
        );
    }

    #[tokio::test]
    async fn test_with_children() {
        let html = html! {
            <HtmlElement tag="button">
                <p>Paragraph text.</p>
            </HtmlElement>
        };

        assert_eq!(
            html,
            String::from("<button data-rsx=\"HtmlElement\"><p>Paragraph text.</p></button>")
        );
    }

    #[tokio::test]
    async fn test_with_data_attributes() {
        let html = html! {
            <HtmlElement
                tag="button"
                attrs=Attrs::with("data-foo", "baz".into())
            >
                <h1>Header text.</h1>
            </HtmlElement>
        };

        assert_eq!(
            html,
            String::from(
                "<button data-foo=\"baz\" data-rsx=\"HtmlElement\"><h1>Header text.</h1></button>"
            )
        );
    }

    #[tokio::test]
    async fn test_with_attrs_with_omit() {
        // Emulate usage by a component
        // Common use case: we want to apply the `class` attribute (or any attribute) manually
        // Then pass the right of the props, omitting `class`.
        let built_props = HtmlElementProps::builder();
        let outer_props = built_props
            .class("THIS_CLASS_SHOULD_BE_OMITTED")
            .id("set-id")
            .role("set-role")
            .build();

        let html = html! {
            <HtmlElement
                class="hard-coded-class"
                attrs=Attrs::from(outer_props).omit(vec!["class"])
            >
                What an awesome element!
            </HtmlElement>
        };

        assert_eq!(
            html,
            String::from(
                "<div class=\"hard-coded-class\" data-rsx=\"HtmlElement\" id=\"set-id\" role=\"set-role\">What an awesome element!</div>"
            )
        );
    }
}
