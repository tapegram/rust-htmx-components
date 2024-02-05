use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use rscx::{component, html, props};
use std::time::{SystemTime, UNIX_EPOCH};

use components::server::{
    attrs::Attrs,
    button::{PrimaryButton, SecondaryButton},
    notification::{
        NoticationCloseButton, NotificationCall, NotificationPresenter, NotificationTransition,
    },
};

pub fn notification_routes() -> Router {
    Router::new()
        .route("/business-logic-example", post(post_business_logic))
        .route("/custom-1", get(get_custom_notification1))
        .route("/custom-2", get(get_custom_notification2))
}

// ### Route Handlers ###

async fn get_custom_notification1() -> Html<String> {
    Html(html! {
        <NotificationPresenter call=NotificationCall::Template>
            <template>
                <div class="bg-white p-10 border">
                    <p>This is a bad notification!</p>
                    <SecondaryButton attrs=Attrs::with("data-toggle-action", "close".into())>
                        Close me
                    </SecondaryButton>
                </div>
            </template>
        </NotificationPresenter>
    })
}

async fn get_custom_notification2() -> Html<String> {
    Html(html! {
        <NotificationPresenter call=NotificationCall::Template>
            <template>
                <NotificationTransition
                    class="bg-white border w-full max-w-sm overflow-hidden shadow-lg"
                >
                    <div class="p-4">
                        <div class="flex items-start">
                            <p class="flex-1">Wow this looks a lil nicer.</p>
                            <NoticationCloseButton />
                        </div>
                    </div>
                </NotificationTransition>
            </template>
        </NotificationPresenter>
    })
}

async fn post_business_logic() -> Html<String> {
    // Do some business logic here...
    // Then we will tell the client to show a success notification.
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    Html(html! {
        {format!("Action complete. Result: {}!", nanos)}
        <NotificationPresenter call=NotificationCall::Success(format!("Server side validated! Answer is {}", nanos)) />
    })
}

// ### Components ###

#[component]
pub fn NotificationsPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">Notifications Playground</h2>
            <div class="flex flex-col gap-4">
                <section>
                    <p><em>Show a toast notification (client-side).</em></p>
                    <div class="flex gap-2">
                        <PrimaryButton
                            onclick="YcControls.showSuccessNotification('Success feels so good!')"
                        >
                            Show Success
                        </PrimaryButton>
                        <PrimaryButton
                            onclick="YcControls.showErrorNotification('This is an error notification.')"
                        >
                            Show Error
                        </PrimaryButton>
                        <PrimaryButton
                            onclick="YcControls.showNotification('This just in', 'You are still not done!')"
                        >
                            Show Generic
                        </PrimaryButton>
                    </div>
                </section>
                <section>
                    <p><em>Show a toast notification (server-side).</em></p>
                    <div class="flex gap-2">
                        <PrimaryButton
                            hx_post="/playground/notifications/business-logic-example"
                        >
                            Show Success
                        </PrimaryButton>
                        <PrimaryButton
                            hx_get="/playground/notifications/custom-1"
                            hx_target="body"
                            hx_swap="beforeend"
                        >
                            Show Custom
                        </PrimaryButton>
                        <PrimaryButton
                        hx_get="/playground/notifications/custom-2"
                        hx_target="body"
                            hx_swap="beforeend"
                        >
                            Show Custom w/ Standard Components
                        </PrimaryButton>
                    </div>
                </section>
            </div>
        </section>
    }
}
