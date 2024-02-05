use rscx::{component, html, props};

#[props]
pub struct CardProps {
    children: String,

    #[builder(default = false)]
    padded: bool,

    #[builder(setter(into), default)]
    class: String,
}

#[component]
pub fn Card(props: CardProps) -> String {
    html! {
        <div class=format!("overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg {}", props.class).trim()>
            <CardContent padded=props.padded>
                {props.children}
            </CardContent>
        </div>
    }
}

#[props]
pub struct CardContentProps {
    children: String,

    #[builder(default = false)]
    padded: bool,
}

#[component]
pub fn CardContent(props: CardContentProps) -> String {
    {
        match props.padded {
            true => html! {
                <div class="px-4 py-5 sm:p-6">
                    {props.children}
                </div>
            },
            false => props.children,
        }
    }
}

#[props]
pub struct CardFooterProps {
    children: String,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> String {
    html! {
        <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
            {props.children}
        </div>
    }
}
