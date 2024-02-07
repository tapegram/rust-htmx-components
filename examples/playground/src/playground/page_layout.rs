use super::appshell::AppShell;
pub use super::appshell::PageHeader;
use htmx_components::server::{modal::ModalLiveRegion, notification::NotificationLiveRegion};
use htmx_components::HtmlLayout;
use rscx::{component, html, props};

#[props]
pub struct PageLayoutProps {
    #[builder(setter(into), default = "Page".into())]
    header: PageHeader,

    #[builder(default)]
    children: String,
}

#[component]
pub fn PageLayout(props: PageLayoutProps) -> String {
    let ctx: crate::playground::context::Context =
        crate::playground::context::context().expect("Unable to retrieve htmx context.");

    if ctx.is_partial_request {
        return props.children;
    }

    html! {
        <HtmlLayout
            head_scripts={
                html! {
                    // Use unminified source for debugging.
                    // <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.js"></script>
                    <script
                        src="https://unpkg.com/htmx.org@1.9.9"
                        integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX"
                        crossorigin="anonymous"
                    ></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js"></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/response-targets.js"></script>
                    <script>{
                        r#"
                        htmx.on("htmx:sendError", function() {
                            YcControls.showErrorNotification("Network Error!");
                        });                
    
                        htmx.on("htmx:responseError", function(error) {
                            YcControls.showErrorNotification(
                                error.detail.xhr.responseText || "Unknown error"
                            );
                        });
    
                        document.addEventListener("htmx:confirm", function(e) {
                            if (!e.target.hasAttribute("hx-confirm")) return true;            
                            e.preventDefault();
                            YcControls.confirm({
                                title: e.target.getAttribute("hx-confirm"),
                                message: e.target.dataset.confirmMessage,
                                actionConfirmed: function() {
                                    e.detail.issueRequest(true);
                                }
                            });
                        });
                        "#
                    }</script>
                }
            }
        >
            <AppShell header=props.header>
                <main hx-ext="loading-states">
                    {props.children}
                </main>
            </AppShell>
            <ModalProxy />
            <div hx-history-elt>
                <NotificationLiveRegion />
                <ModalLiveRegion />
            </div>
        </HtmlLayout>
    }
}

#[component]
fn ModalProxy() -> String {
    let ctx: crate::playground::context::Context =
        crate::playground::context::context().expect("Unable to retrieve htmx context.");

    let query_params = ctx.page_query_params;

    let modal = query_params
        .get("modal")
        .map(|s| if s.is_empty() { None } else { Some(s) });

    match modal {
        Some(Some(modal)) => {
            let modal_url = if modal.starts_with('/') {
                modal.to_owned()
            } else {
                format!("{}/{}", &ctx.page_url, &modal)
            };

            html! {
                <div hx-get=modal_url hx-trigger="load" />
            }
        }
        _ => html! { <></> },
    }
}
