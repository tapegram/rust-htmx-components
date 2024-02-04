use super::attrs::Attrs;
use super::transition::Transition;
use super::yc_control::YcControl;
use rscx::{component, html, props};

#[props]
pub struct FlyoutProps {
    #[builder(setter(into))]
    title: String,
    children: String,
}

#[component]
pub fn Flyout(props: FlyoutProps) -> String {
    html! {
        <YcControl
            control="flyout"
            class="relative z-8"
            role="dialog"
            aria_labelledby="slide-over-title"
            attrs=Attrs::with("aria-modal", "true".into())
        >
            // <!-- Background backdrop, show/hide based on slide-over state. -->
            <div class="fixed inset-0"></div>
            <div class="fixed inset-0 overflow-hidden">
                <div class="absolute inset-0 overflow-hidden">
                    <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16">
                        <Transition
                            class="pointer-events-auto w-screen max-w-2xl"
                            enter="transform transition ease-in-out duration-500 sm:duration-700"
                            enter_from="translate-x-full"
                            enter_to="translate-x-0"
                            leave="transform transition ease-in-out duration-500 sm:duration-700"
                            leave_from="translate-x-0"
                            leave_to="translate-x-full"
                            attrs=Attrs::with("data-flyout-panel", "true".into())
                        >
                            <div class="flex h-full flex-col overflow-y-scroll bg-white py-6 shadow-xl">
                                <div class="px-4 sm:px-6">
                                    <div class="flex items-start justify-between">
                                        <h2 class="text-base font-semibold leading-6 text-gray-900" id="slide-over-title">{props.title}</h2>
                                        <div class="ml-3 flex h-7 items-center">
                                            <button type="button" data-toggle-action="close" class="relative rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                                                <span class="absolute -inset-2.5"></span>
                                                <span class="sr-only">Close panel</span>
                                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                                <div class="relative mt-6 flex-1 px-4 sm:px-6">
                                    {props.children}
                                </div>
                            </div>
                        </Transition>
                    </div>
                </div>
            </div>
        </YcControl>
    }
}
