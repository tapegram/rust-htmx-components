use rscx::{component, html, props, CollectFragment};
use typed_builder::TypedBuilder;

use web_macros::*;

use super::attrs::Attrs;
use super::html_element::HtmlElement;
use super::opt_attrs::opt_attrs;
use super::transition::Transition;
use super::yc_control::Toggle;

pub enum MenuSize {
    Small,
    Medium,
}

#[derive(TypedBuilder)]
pub struct MenuLink {
    #[builder(setter(into))]
    label: String,

    #[builder(setter(into), default="".into())]
    sr_suffix: String,

    #[builder(default=Attrs::default())]
    attrs: Attrs,
}

impl From<(String, String)> for MenuLink {
    fn from((label, href): (String, String)) -> Self {
        Self {
            label,
            sr_suffix: "".into(),
            attrs: Attrs::with("href", href),
        }
    }
}

#[props]
pub struct PopupMenuProps {
    #[builder(setter(into))]
    id: String,

    #[builder(setter(into), default)]
    class: String,

    #[builder(setter(into), default)]
    button_class: String,
    button_content: String,

    #[builder(default=MenuSize::Medium)]
    size: MenuSize,

    children: String,
}

#[component]
pub fn PopupMenu(props: PopupMenuProps) -> String {
    html! {
        <Toggle class=format!("relative {}", props.class).trim()>
            <div>
                <button
                    type="button"
                    id=format!("{}-button", &props.id)
                    class=format!("relative {}", props.button_class).trim()
                    aria-expanded="false"
                    aria-haspopup="true"
                    data-toggle-action="click"
                >
                    <span class="absolute -inset-1.5"></span>
                    <span class="sr-only">Open menu</span>
                    {props.button_content}
                </button>
            </div>
            <Transition
                class={
                    let m_width = match props.size {
                        MenuSize::Small => "w-32".to_string(),
                        MenuSize::Medium => "w-48".to_string(),
                    };
                    format!("absolute right-0 z-10 mt-2 {} origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none", m_width)
                }
                role="menu"
                aria_orientation="vertical"
                aria_labelledby=format!("{}-button", &props.id)
                tabindex="-1"
                enter="transition ease-out duration-200"
                enter_from="transform opacity-0 scale-95"
                enter_to="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leave_from="transform opacity-100 scale-100"
                leave_to="transform opacity-0 scale-95"
            >
                {props.children}
            </Transition>
        </Toggle>
    }
}

#[html_element]
pub struct MenuItemProps {
    #[builder(setter(into))]
    title: String,

    #[builder(setter(into), default)]
    sr_suffix: String,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> String {
    html! {
        <HtmlElement
            tag="a"
            class={
                if props.class.is_empty() { "cursor-pointer block px-4 py-2 text-sm text-gray-700 hover:bg-gray-50" }
                else { &props.class }
            }
            role="menuitem"
            tabindex="-1"
            attrs=spread_attrs!(props | omit(class))
        >
            {props.title}
            <span class="sr-only">{props.sr_suffix}</span>
        </HtmlElement>
    }
}

#[props]
pub struct MenuProps {
    #[builder(setter(into))]
    id: String,

    links: Vec<MenuLink>,
}

#[component]
pub fn Menu(props: MenuProps) -> String {
    #[allow(unused_braces)]
    props
        .links
        .into_iter()
        .enumerate()
        .map(
            |(
                i,
                MenuLink {
                    label,
                    sr_suffix,
                    attrs,
                },
            )| {
                html! {
                    <a
                        class="block px-4 py-2 text-sm text-gray-700 cursor-pointer hover:bg-gray-50"
                        role="menuitem"
                        tabindex="-1"
                        id={format!("{}-item-{}", &props.id, i)}
                        {opt_attrs(attrs.to_hashmap())}
                    >
                        {label}
                        <span class="sr-only">", "{sr_suffix}</span>
                    </a>
                }
            },
        )
        .collect_fragment()
}
