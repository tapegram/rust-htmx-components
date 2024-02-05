use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct TransitionProps {
    #[builder(setter(into))]
    enter: String,

    #[builder(setter(into))]
    enter_from: String,

    #[builder(setter(into))]
    enter_to: String,

    #[builder(setter(into))]
    leave: String,

    #[builder(setter(into))]
    leave_from: String,

    #[builder(setter(into))]
    leave_to: String,

    #[builder(setter(into), default)]
    children: String,

    #[builder(setter(into), default=String::from("div"))]
    tag: String,
}

#[component]
pub fn Transition(props: TransitionProps) -> String {
    html! {
        <HtmlElement
            tag=props.tag
            class=format!("hidden {}", props.class)
            component_name="Transition"
            attrs=spread_attrs!(props | omit(class))
                .set("data-yc-control", "transition".into())
                .set("data-transition-enter", props.enter)
                .set("data-transition-enter-start", props.enter_from)
                .set("data-transition-enter-end", props.enter_to)
                .set("data-transition-leave", props.leave)
                .set("data-transition-leave-start", props.leave_from)
                .set("data-transition-leave-end", props.leave_to)
        >
            {props.children}
        </HtmlElement>
    }
}
