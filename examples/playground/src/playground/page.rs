#![allow(unused_braces)]
use axum::{response::Html, routing::get, Router};
use rscx::{component, html, props};

use crate::playground::page_layout::PageLayout;

pub fn page_routes() -> Router {
    Router::new().route("/render", get(get_test_render))
}

// ### Route Handlers ###

// Test to see if we can partial render a component that includes PageLayout.
async fn get_test_render() -> Html<String> {
    Html(html! {
        <PageLayout>
            <section>
                <h1>Test Render</h1>
                <p>
                    "If you are viewing this page at the url `/playground/page/render`
                    you should see the full render (header and footer)"
                </p>
                <p>
                    "If this is being pulled in from an htmx request
                    we should just see the `section` tag only."
                </p>
            </section>
        </PageLayout>
    })
}

// ### Components ###

#[component]
pub fn PagePlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">"Test rendering"</h2>
            <ul class="list-disc list-inside">
                <li>
                    <a
                        class="text-blue-600 hover:underline"
                        href="/playground/page/render"
                    >
                        "Goto a full page render."
                    </a>
                </li>
                <li>
                    <a
                        class="text-blue-600 hover:underline"
                        hx-get="/playground/page/render"
                        hx-target=".partial-rendered-content"
                    >
                        "See a partial render."
                    </a>
                </li>
            </ul>
            <div class="text-sm italic partial-rendered-content"></div>
        </section>
    }
}
