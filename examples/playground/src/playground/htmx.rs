use axum::{response::Html, routing::get, Router};
use rscx::{component, html, props};

use htmx_components::server::button::SecondaryButton;

pub fn htmx_routes() -> Router {
    Router::new().route("/", get(htmx_test))
}

// ### Route Handlers ###

async fn htmx_test() -> Html<String> {
    Html("Is this the real life? Is this just fantasy?".into())
}

// ### Components ###

#[component]
pub fn HtmxPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">HTMX Rendering</h2>
            <div class="flex gap-2">
                <SecondaryButton
                    hx_get="/playground/htmx"
                    hx_swap="outerHTML"
                >
                    Click me!
                </SecondaryButton>
            </div>
        </section>
    }
}
