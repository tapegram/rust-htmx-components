use rscx::{component, html, props, CollectFragment};

use super::html_element::HtmlElement;

use web_macros::*;

pub enum TableHeading {
    Title(String),
    Empty(String),
}

impl TableHeading {
    pub fn title(title: impl Into<String>) -> TableHeading {
        TableHeading::Title(title.into())
    }
    pub fn empty(sr_only_text: impl Into<String>) -> TableHeading {
        TableHeading::Empty(sr_only_text.into())
    }
}

pub type TableHeadings = Vec<TableHeading>;

#[props]
pub struct TableProps {
    headings: TableHeadings,
    body: Vec<String>,
}

#[component]
pub fn Table(props: TableProps) -> String {
    html! {
        <table class="min-w-full divide-y divide-gray-300">
            <TableHeadingsRow headings=props.headings />
            <TableBody body=props.body />
        </table>
    }
}

pub enum TDVariant {
    Default,
    First,
    Last,
    LastNonEmptyHeading,
}

#[props]
pub struct TableDataProps {
    children: String,

    #[builder(default=TDVariant::Default)]
    variant: TDVariant,
}

#[component]
pub fn TableData(props: TableDataProps) -> String {
    let td_class = match props.variant {
        TDVariant::Default => "whitespace-nowrap px-3 py-4 text-sm text-gray-500",
        TDVariant::First => {
            "whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6"
        }
        TDVariant::Last => {
            "relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6"
        }
        TDVariant::LastNonEmptyHeading => {
            "whitespace-nowrap py-4 pl-3 pr-4 text-left text-sm font-medium sm:pr-6"
        }
    };

    html! {
        <td class=td_class>{props.children}</td>
    }
}

#[component]
fn TableHeadingsRow(headings: TableHeadings) -> String {
    html! {
        <thead class="bg-gray-50">
            <tr>
            {headings.iter().enumerate().map(|(i, heading)| {
                let th_class = match i {
                    // first heading:
                    0 => "py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6",

                    // last heading:
                    _ if i == headings.len() - 1 => "py-3.5 pl-3 pr-4 text-left text-sm font-semibold text-gray-900 sm:pr-6",

                    // middle headings:
                    _ => "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                };

                match heading {
                    TableHeading::Title(heading) => html! {
                        <th scope="col" class=th_class>{heading}</th>
                    },
                    TableHeading::Empty(sr_only_text) => html! {
                        <th scope="col" class=format!("relative {}", th_class)>
                            <span class="sr-only">{sr_only_text}</span>
                        </th>
                    },
                }
            }).collect_fragment()}
            </tr>
        </thead>
    }
}

#[component]
fn TableBody(body: Vec<String>) -> String {
    html! {
        <tbody class="divide-y divide-gray-200 bg-white">
            {
                body.iter().map(|row| html! {
                    <tr data-loading-states>{row}</tr>
                })
                .collect_fragment()
            }
        </tbody>
    }
}

#[html_element]
pub struct ActionLinkProps {
    children: String,
    #[builder(setter(into))]
    sr_text: String,
}

#[component]
pub fn ActionLink(props: ActionLinkProps) -> String {
    html! {
        <HtmlElement
            tag="a"
            class=format!("cursor-pointer text-indigo-600 hover:text-indigo-900, {}", props.class).trim()
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}<span class="sr-only">{props.sr_text}</span>
        </HtmlElement>
    }
}

#[derive(Clone)]
pub struct Confirm {
    pub title: String,
    pub message: String,
}

#[html_element]
pub struct DeleteActionLinkProps {
    children: String,
    confirm: Confirm,

    #[builder(setter(into))]
    sr_text: String,

    #[builder(default = false)]
    show_loader_on_delete: bool,
}

#[component]
pub fn DeleteActionLink(props: DeleteActionLinkProps) -> String {
    html! {
        <ActionLink
            sr_text=props.sr_text
            attrs=spread_attrs!(props)
                .set("hx-confirm", props.confirm.title)
                .set("data-confirm-message", props.confirm.message)
                .set_if("data-loading-disable", "true".into(), props.show_loader_on_delete)
        >
            {if props.show_loader_on_delete {
                html! {
                    <div class="htmx-indicator inline-flex animate-spin mr-2 items-center justify-center rounded-full w-4 h-4 bg-gradient-to-tr from-gray-500 to-white">
                        <span class="inline h-3 w-3 rounded-full bg-white hover:bg-gray-50"></span>
                    </div>
                }
            } else { String::from("") }}
            {props.children}
        </ActionLink>
    }
}

#[component]
pub fn TableDataActions(children: String) -> String {
    html! {
        <div class="inline-flex gap-4">
            {children}
        </div>
    }
}
