use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct PageHeaderToolbarProps {
    #[builder(setter(into))]
    title: String,
    buttons: String,
}

#[component]
pub fn PageHeaderToolbar(props: PageHeaderToolbarProps) -> String {
    html! {
        <HtmlElement
            tag="header"
            class=props.class
            component_name="PageHeaderToolbar"
            attrs=spread_attrs!(props | omit(class))
        >
            <div class="mt-2 md:flex md:items-center md:justify-between">
                <div class="min-w-0 flex-1">
                    <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">{props.title}</h2>
                </div>
                <div class="mt-4 flex flex-shrink-0 gap-2 md:ml-4 md:mt-0">
                    {props.buttons}
                </div>
            </div>
        </HtmlElement>
    }
}
