use super::transition::Transition;
use super::yc_control::YcControlJsApi;
use rscx::{component, html, props, CollectFragmentAsync};

/**
 * NotificationLiveRegion
 *
 * Holds all attached notifications to show
 * Also contains all standard templates.
 */

#[component]
pub fn NotificationLiveRegion() -> String {
    html! {
        <div id="notification-live-region" aria-live="assertive" class="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6">
            <section class="flex w-full flex-col items-center space-y-4 sm:items-end" data-notification-content>
            </section>
            <template id="tpl-notification">
                <SimpleNotification icon_svg=IconSvg::Info />
            </template>
            <template id="tpl-notification-icons">
                <NotificationIcon svg=IconSvg::Success/>
                <NotificationIcon svg=IconSvg::Error/>
                <NotificationIcon svg=IconSvg::Info/>
                // Add any additional prerendered icons here.
            </template>
        </div>
    }
}

#[props]
pub struct SimpleNotificationProps {
    #[builder(setter(into), default="Notification".to_string())]
    title: String,

    #[builder(setter(into), default)]
    message: String,

    #[builder(setter(into))]
    icon_svg: IconSvg,
}

#[component]
pub fn SimpleNotification(props: SimpleNotificationProps) -> String {
    html! {
        <NotificationTransition
            class="w-full max-w-sm overflow-hidden rounded-lg bg-white shadow-lg ring-1 ring-black ring-opacity-5"
        >
            <div class="p-4">
                <div class="flex items-start">
                    <div class="flex-shrink-0">
                        <NotificationIcon svg=props.icon_svg />
                    </div>
                    <div class="ml-3 w-0 flex-1 pt-0.5">
                        <p class="text-sm font-medium text-gray-900" data-notification-title>{props.title}</p>
                        <p class="mt-1 text-sm text-gray-500" data-notification-message>{props.message}</p>
                    </div>
                    <NoticationCloseButton />
                </div>
            </div>
        </NotificationTransition>
    }
}

pub enum IconSvg {
    Success,
    Error,
    Info,
    Custom(String),
}

impl From<String> for IconSvg {
    fn from(s: String) -> Self {
        IconSvg::Custom(s)
    }
}

#[props]
struct NotificationIconProps {
    svg: IconSvg,
}

#[component]
fn NotificationIcon(props: NotificationIconProps) -> String {
    match props.svg {
        IconSvg::Success => html! {
            <svg class="h-6 w-6 text-green-400" data-notification-icon="success" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        },
        IconSvg::Error => html! {
            <svg class="h-6 w-6 text-red-400" data-notification-icon="error" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
        },
        IconSvg::Info => html! {
            <svg class="h-6 w-6 text-blue-400" data-notification-icon="info" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
            </svg>
        },
        IconSvg::Custom(svg) => svg,
    }
}

// #### Notification components for use with axum resources. ###############

/**
 * NotificationFlashes
 *
 * Use this to present a axum_flash message with a notification
 */

#[props]
pub struct NotificationFlashesProps {
    flashes: axum_flash::IncomingFlashes,
}

#[component]
pub fn NotificationFlashes(props: NotificationFlashesProps) -> String {
    props
        .flashes
        .into_iter()
        .map(|(level, message)| async move {
            let js_notification_fn = match level {
                axum_flash::Level::Success => "showSuccessNotification",
                axum_flash::Level::Error => "showErrorNotification",
                _ => "showErrorNotification", // TODO! Replace with generic notification.
            };

            let message = serde_json::to_string(&message).unwrap();

            html! {
                <YcControlJsApi call=format!("{}({})", js_notification_fn, message) />
            }
        })
        .collect_fragment_async()
        .await
}

pub enum NotificationCall {
    Success(String),
    Error(String),
    Info(String, String), // title, message
    Template,
    TemplateSelector(String),
}

/**
 * NotificationPresenter
 *
 * Use this to present a notification from a server resource.
 */

#[props]
pub struct NotificationPresenterProps {
    call: NotificationCall,

    #[builder(default)]
    children: String,
}

fn js_enc<T>(data: &T) -> String
where
    T: ?Sized + serde::Serialize,
{
    serde_json::to_string::<T>(data).unwrap()
}

#[component]
pub fn NotificationPresenter(props: NotificationPresenterProps) -> String {
    let api_call = match props.call {
        NotificationCall::Success(message) => {
            format!("showSuccessNotification({})", js_enc(&message))
        }
        NotificationCall::Error(message) => {
            format!("showErrorNotification({})", js_enc(&message))
        }
        NotificationCall::Info(title, message) => {
            format!("showNotification({}, {})", js_enc(&title), js_enc(&message))
        }
        NotificationCall::TemplateSelector(templateSelector) => {
            format!(
                "showNotificationWithTemplate({})",
                js_enc(&templateSelector),
            )
        }
        NotificationCall::Template => {
            if props.children.is_empty() {
                panic!("NotificationPresenter: Template call requires children.")
            }
            "showNotificationWithTemplate(callerScript.nextElementSibling)".into()
        }
    };

    html! {
        <YcControlJsApi call=api_call />
        {props.children}
    }
}

// #### Notification components to help you build your own. ###############

#[props]
pub struct NotificationTransitionProps {
    #[builder(setter(into), default="".to_string())]
    class: String,

    #[builder(default)]
    children: String,
}

#[component]
pub fn NotificationTransition(props: NotificationTransitionProps) -> String {
    html! {
        <Transition
            class=format!("pointer-events-auto {}", props.class).trim()
            enter="transform ease-out duration-300 transition"
            enter_from="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
            enter_to="translate-y-0 opacity-100 sm:translate-x-0"
            leave="transition ease-in duration-300"
            leave_from="opacity-100"
            leave_to="opacity-0"
        >
            {props.children}
        </Transition>
    }
}

#[component]
pub fn NoticationCloseButton() -> String {
    html! {
        <div class="ml-4 flex flex-shrink-0">
            <button type="button" data-toggle-action="close" data-notification-close class="inline-flex rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                <span class="sr-only">Close</span>
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-notification-close>
                    <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                </svg>
            </button>
        </div>
    }
}
