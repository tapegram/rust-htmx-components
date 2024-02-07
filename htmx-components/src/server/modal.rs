use super::transition::Transition;
use super::yc_control::YcControl;
use crate::server::attrs::Attrs;
use rscx::{component, html, props};

const MODALS_ID: &str = "modal-live-region";
pub fn modal_target() -> String {
    format!("#{}", MODALS_ID)
}

pub enum ModalSize {
    Small,
    Medium,
    Large,
    SmallScreen,
    MediumScreen,
    Custom(String),
}

#[props]
pub struct ModalProps {
    #[builder(default = ModalSize::Medium)]
    size: ModalSize,

    #[builder(setter(into))]
    children: String,
}

#[component]
pub fn Modal(props: ModalProps) -> String {
    html! {
        <YcControl
            control="modal"
            class="relative z-10"
            role="dialog"
            aria_labelledby="modal-title"
            attrs=Attrs::with("aria-modal", "true".into())
        >
            <Transition
                class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
                enter="ease-out duration-300"
                enter_from="opacity-0"
                enter_to="opacity-100"
                leave="ease-in duration-200"
                leave_from="opacity-100"
                leave_to="opacity-0"
            />
            <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <Transition
                        class={
                            let m_width = match props.size {
                                ModalSize::Small => "sm:max-w-sm".to_string(),
                                ModalSize::Medium => "sm:max-w-md".to_string(),
                                ModalSize::Large => "sm:max-w-lg".to_string(),
                                ModalSize::SmallScreen => "sm:max-w-screen-sm".to_string(),
                                ModalSize::MediumScreen => "sm:max-w-screen-md".to_string(),
                                ModalSize::Custom(width) => width,
                            };
                            format!("relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 w-full {} sm:p-6", m_width)
                        }
                        attrs=Attrs::with("data-modal-panel", "true".into())
                        enter="ease-out duration-300"
                        enter_from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        enter_to="opacity-100 translate-y-0 sm:scale-100"
                        leave="ease-in duration-200"
                        leave_from="opacity-100 translate-y-0 sm:scale-100"
                        leave_to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        >
                            <div>
                                {props.children}
                            </div>
                    </Transition>
                </div>
            </div>
        </YcControl>
    }
}

// TODO! Provide option to show close "X" link in modal
/*
    <div class="absolute right-0 top-0 hidden pr-4 pt-4 sm:block">
        <button type="button" class="rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
            <span class="sr-only">Close</span>
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
        </button>
    </div>
*/

#[component]
pub fn ConfirmDeleteModal() -> String {
    html! {
        <Modal>
            <div class="sm:flex sm:items-start">
                <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                    <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                    </svg>
                </div>
                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                    <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title" data-confirm-delete-title>Deactivate account</h3>
                    <div class="mt-2">
                        <p class="text-sm text-gray-500" data-confirm-delete-message>Are you sure you want to delete this item?.</p>
                    </div>
                </div>
            </div>
            <div class="mt-5 sm:mt-4 sm:flex sm:flex-row-reverse">
                <button data-toggle-action="close" data-confirm-action="delete" type="button" class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto">Delete</button>
                <button data-toggle-action="close" type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto">Cancel</button>
            </div>
        </Modal>
    }
}

/**
 * ModalLiveRegion
 *
 * Holds all pre-rendered modals to to be rendered client-side by Modals control.
 */
#[component]
pub fn ModalLiveRegion() -> String {
    html! {
      <div id=MODALS_ID>
          <section data-modal-content>
          </section>
          <template id="tpl-confirm-delete-modal">
              <ConfirmDeleteModal />
          </template>
      </div>
    }
}
