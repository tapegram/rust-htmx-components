use axum::{response::Html, routing::get, Router};
use rscx::{component, html, props};

use file_input::{file_input_routes, FileInputPlayground};
use form::FormPlayground;
use html_element::HtmlElementPlayground;
use htmx::{htmx_routes, HtmxPlayground};
use modal::{modal_routes, ModalPlayground};
use notifications::{notification_routes, NotificationsPlayground};
use page::{page_routes, PagePlayground};

use crate::playground::page_layout::PageLayout;

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

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_playground))
        .nest("/page", page_routes())
        .nest("/htmx", htmx_routes())
        .nest("/modals", modal_routes())
        .nest("/notifications", notification_routes())
        .nest("/file-input", file_input_routes())
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
