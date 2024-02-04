use rscx::{component, html, props};

use web_macros::*;

use super::{attrs::Attrs, html_element::HtmlElement};
use crate::server::yc_control::YcControl;

#[html_element]
pub struct TextInputProps {
    #[builder(setter(into), default="text".into())]
    input_type: String,

    #[builder(setter(into), default=None)]
    error: Option<String>,
}

#[component]
pub fn TextInput(props: TextInputProps) -> String {
    let class = match props.error {
        Some(_) => "bg-red-50 ring-red-500 text-red-500 placeholder-red-700 focus:ring-red-500 focus:border-red-500",
        None => "text-gray-900 ring-gray-300 placeholder:text-gray-400 focus:ring-indigo-600",
    };

    let (tag, children) = match props.input_type.as_str() {
        "textarea" => ("textarea", props.value.clone()),
        _ => ("input", "".to_string()),
    };

    html! {
        <HtmlElement
            tag=tag
            id=props.name.clone()
            class=format!("block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 ring-inset focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6 {}", class)
            attrs=spread_attrs!(props | omit(id, class)).set("type", props.input_type.clone())
            children=children
        />
        <ErrorMessage message=props.error />
    }
}

#[html_element]
pub struct LabelProps {
    #[builder(setter(into))]
    for_input: String,
    children: String,

    #[builder(default = false)]
    error: bool,
}

#[component]
pub fn Label(props: LabelProps) -> String {
    let color = if props.error {
        "text-red-600 dark:text-red-500"
    } else {
        "text-gray-900"
    };

    html! {
        <HtmlElement
            tag="label"
            class=format!("block text-sm font-medium leading-6 {} {}", color, props.class).trim()
            attrs=spread_attrs!(props | omit(class)).set("for", props.for_input)
        >
            {props.children}
        </HtmlElement>
    }
}

#[html_element]
pub struct SelectProps {
    children: String,

    #[builder(setter(into), default=None)]
    error: Option<String>,
}

#[component]
pub fn Select(props: SelectProps) -> String {
    let class = match props.error {
        Some(_) => "bg-red-50 ring-red-500 text-red-500 placeholder-red-700 focus:ring-red-500 focus:border-red-500",
        None => "text-gray-900 ring-gray-300 focus:ring-indigo-600",
    };
    html! {
        <HtmlElement
            tag="select"
            id=props.name.clone()
            class=format!("block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm sm:leading-6 {} {}", class, props.class).trim()
            attrs=spread_attrs!(props | omit(id, class))
        >
            {props.children}
        </HtmlElement>
        <ErrorMessage message=props.error />
    }
}

#[html_element]
pub struct SelectOptionProps {
    #[builder(setter(into), default)]
    label: String,

    #[builder(default = false)]
    selected: bool,

    #[builder(default)]
    children: String,
}

#[component]
pub fn SelectOption(props: SelectOptionProps) -> String {
    html! {
        <HtmlElement
            tag="option"
            id=props.name.clone()
            attrs=spread_attrs!(props | omit(id))
                .set_if("selected", "selected".into(), props.selected)
                .set_if("label", props.label.clone(), !props.label.is_empty())
        >
            {props.children}
        </HtmlElement>
    }
}

#[component]
fn ErrorMessage(message: Option<String>) -> String {
    if let Some(message) = message {
        html! {
            <p class="text-sm text-red-600 dark:text-red-500">{message}</p>
        }
    } else {
        String::new()
    }
}

#[html_element]
pub struct ButtonProps {
    #[builder(setter(into), default="button".into())]
    kind: String,
    children: String,
}

#[component]
pub fn Button(props: ButtonProps) -> String {
    let button_type = props.kind.clone();
    let css = match props.kind.as_str() {
        "submit" => "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
        _ => "text-sm font-semibold leading-6 text-gray-900",
    };

    html! {
        <HtmlElement
            tag="button"
            class=format!("{} {}", css, props.class).trim()
            attrs=spread_attrs!(props | omit(class, name)).set("type", button_type)
        >
            {props.children}
        </HtmlElement>
    }
}

#[props]
pub struct FileInputProps {
    #[builder(setter(into))]
    id: String,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into), default = "Any file up to 10MB".into())]
    file_hint_message: String,

    #[builder(setter(into), default)]
    accept: String,

    #[builder(default = false)]
    multiple: bool,
}

#[component]
pub fn FileInput(props: FileInputProps) -> String {
    html! {
        <YcControl
            control="file-input"
            class="mt-2 group flex justify-center transition-all rounded-lg border border-dashed border-gray-900/25 px-6 py-10 data-[dragover]:border-2 data-[dragover]:border-indigo-600/50 data-[dragover]:bg-gray-900/10"
        >
            <div class="text-center">
                <svg class="mx-auto h-12 w-12 text-gray-300" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M1.5 6a2.25 2.25 0 012.25-2.25h16.5A2.25 2.25 0 0122.5 6v12a2.25 2.25 0 01-2.25 2.25H3.75A2.25 2.25 0 011.5 18V6zM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0021 18v-1.94l-2.69-2.689a1.5 1.5 0 00-2.12 0l-.88.879.97.97a.75.75 0 11-1.06 1.06l-5.16-5.159a1.5 1.5 0 00-2.12 0L3 16.061zm10.125-7.81a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0z" clip-rule="evenodd" />
                </svg>
                <div class="mt-4 flex text-sm leading-6 text-gray-600">
                    <label for=props.id.as_ref() class="relative cursor-pointer rounded-md font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500">
                        <span>Upload a file</span>
                        <input
                            type="file"
                            id=props.id.as_ref()
                            name=props.name.as_ref()
                            class="sr-only"
                            {
                                String::from(
                                    Attrs::default()
                                        .set_if("accept", props.accept.clone(), !props.accept.is_empty())
                                        .set_if("multiple", "true".into(), props.multiple)
                                )
                            }
                        />
                    </label>
                    <p class="pl-1">or drag and drop</p>
                </div>
                <p class="text-xs leading-5 text-gray-600">
                    <span class="group-[.file-selected]:hidden">{props.file_hint_message}</span>
                    <span class="hidden font-bold text-sm group-[.file-selected]:inline" data-file-input-selected-message>File Selected!</span>
                </p>
            </div>
        </YcControl>
    }
}

// FormLayouts ////////////////////////////////////////////////

#[html_element]
pub struct GridLayoutProps {
    children: String,
}

#[component]
pub fn GridLayout(props: GridLayoutProps) -> String {
    html! {
        <HtmlElement
            tag="div"
            class=format!("grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 {}", props.class).trim()
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}
        </HtmlElement>
    }
}

#[derive(Clone)]
pub enum CellSpan {
    Size(usize),
    Full,
}

impl From<usize> for CellSpan {
    fn from(size: usize) -> Self {
        CellSpan::Size(size)
    }
}

#[html_element]
pub struct GridCellProps {
    children: String,

    #[builder(setter(into), default=CellSpan::Full)]
    span: CellSpan,

    #[builder(default = 0)]
    start: usize,
}

#[component]
pub fn GridCell(props: GridCellProps) -> String {
    html! {
        <HtmlElement
            tag="div"
            class={
                let mut classes = Vec::new();

                // For now hardcode this layout of cells (col w/ .5rem gap)
                // If we have other cell layouts, we can create new enum
                classes.push("flex flex-col gap-2".to_string());

                classes.push(match props.span {
                    // generates classes (for tailwind) in tailwind.config.js safelist
                    CellSpan::Size(size) => format!("sm:col-span-{}", size),
                    CellSpan::Full => "sm:col-span-full".to_string(),
                });

                if props.start > 0 {
                    // generates classes (for tailwind) in tailwind.config.js safelist
                    classes.push(format!("sm:col-start-{}", props.start));
                }

                if !props.class.is_empty() {
                    classes.push(props.class);
                }

                classes.join(" ")
            }
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}
        </HtmlElement>
    }
}
