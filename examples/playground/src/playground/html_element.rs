#![allow(unused_braces)]
use components::server::html_element::HtmlElement;
use rscx::{component, html, props};

#[component]
pub fn HtmlElementPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">"HtmlElement Playground"</h2>
            <div class="flex flex-col gap-4">
                <SimpleElement class="font-bold" simple="YO".into()>
                    Simple but not so simple.
                </SimpleElement>
                <FooButton />
                <MessageButton message="This is a message from a button!".into()>
                    I am a MessageButton, click to see a message!
                </MessageButton>
            </div>
        </section>
    }
}

#[component]
fn FooButton() -> String {
    html! {
        <HtmlElement
            tag="button"
            class="bg-slate-200 ml-4 p-3 rounded-full"
            id="btn-foo"
        >
            A button rendered w/ HTMLElement. Click for more foo!
        </HtmlElement>
    }
}

#[props]
struct MessageButtonProps {
    #[builder(default)]
    message: String,

    #[builder(default)]
    children: String,
}

#[component]
fn MessageButton(props: MessageButtonProps) -> String {
    html! {
        <HtmlElement
            tag="button"
            id="btn-alert"
            class="bg-slate-200 ml-4 p-3 rounded-full"
            onclick={format!("alert('{}')", props.message)}
        >
            {props.children}
        </HtmlElement>
    }
}

// This macro adds all standard HTML attributes for your component!
#[web_macros::html_element]
pub struct SimpleElementProps {
    #[builder(default)]
    children: String,

    #[builder(default="SIMPLE!".to_string())]
    simple: String,

    #[builder(setter(into), default=String::from("div"))]
    tag: String,
}

#[component]
fn SimpleElement(props: SimpleElementProps) -> String {
    html! {
        <div class=props.class data-simple=props.simple data-tag=props.tag>
            <p>I am foo, hear me roar!</p>
            <div>{props.children}</div>
        </div>
    }
}
