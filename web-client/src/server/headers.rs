use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct SecondaryHeaderProps {
    #[builder(setter(into))]
    title: String,

    #[builder(setter(into), default)]
    subtitle: String,

    #[builder(default)]
    children: String,

    #[builder(setter(into), default=String::from("header"))]
    tag: String,
}

#[component]
pub fn SecondaryHeader(props: SecondaryHeaderProps) -> String {
    html! {
        <HtmlElement
            tag=props.tag
            attrs=spread_attrs!(props)
        >
            <h2
                class="text-lg font-medium leading-6 text-gray-900"
            >
                {props.title}
            </h2>
            {
                if !props.subtitle.is_empty() {
                    html! {
                        <p class="mt-1 text-sm text-gray-500">{props.subtitle}</p>
                    }
                } else {
                    "".into()
                }
            }
            {props.children}
        </HtmlElement>
    }
}
