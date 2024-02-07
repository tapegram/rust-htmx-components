use axum::{http::StatusCode, middleware, response::Html, routing::get, Router};
use rscx::{component, html, props};

use file_input::{file_input_routes, FileInputPlayground};
use form::FormPlayground;
use html_element::HtmlElementPlayground;
use htmx::{htmx_routes, HtmxPlayground};
use modal::{modal_routes, ModalPlayground};
use notifications::{notification_routes, NotificationsPlayground};
use page::{page_routes, PagePlayground};

use crate::playground::page_layout::PageLayout;

use self::{context::provide_context_layer, state::WebHtmxState};

pub mod appshell;
pub mod context;
pub mod file_input;
pub mod form;
pub mod html_element;
pub mod htmx;
pub mod modal;
pub mod notifications;
pub mod page;
pub mod page_layout;
pub mod state;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .with_state(state.clone())
        .route("/", get(get_playground))
        .nest("/page", page_routes())
        .nest("/htmx", htmx_routes())
        .nest("/modals", modal_routes())
        .nest("/notifications", notification_routes())
        .nest("/file-input", file_input_routes())
        .layer(middleware::from_fn_with_state(state, provide_context_layer))
}

// ### Route Handlers ###

async fn get_playground() -> Html<String> {
    Html(html! {
        <PageLayout header="Component Playground">
            <PlaygroundPgContent />
        </PageLayout>
    })
}

// ### Components ###

#[component]
pub fn PlaygroundPgContent() -> String {
    html! {
        <section>
            <h1 class="text-xl text-slate-600">Yall Ready for This?</h1>
            <marquee>
                "It's The Playground&#133; Let's have some fun!"
            </marquee>
        </section>
        <NotificationsPlayground />
        <ModalPlayground />
        <FileInputPlayground />
        <FormPlayground />
        <HtmxPlayground />
        <PagePlayground />
        <HtmlElementPlayground />
    }
}
