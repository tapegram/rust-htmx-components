use once_cell::sync::Lazy;
use rscx::{component, html, props};
use std::time::{SystemTime, UNIX_EPOCH};

// TEMP HACK! Used to bust cache on client scripts and stylesheets.
// TODO Get hash of each build file and use that.
static TS: Lazy<u128> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
});

#[props]
pub struct HtmlLayoutProps {
    #[builder(setter(into), default = "Yall Chart".to_string())]
    head_title: String,

    #[builder(default)]
    head_links: String,

    #[builder(default)]
    head_scripts: String,

    #[builder(default)]
    children: String,
}

/**
* Note, for now, im using a CDN for tailwindcss. This is not ideal for productionn -- but
* theres a bunch to figure out how to get this working well as a library, what with all
* the tailwind building and themes and junk.
*
*/
#[component]
pub fn HtmlLayout(props: HtmlLayoutProps) -> String {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{props.head_title}</title>
                <script src="https://cdn.tailwindcss.com"></script>
                <script>{
                    "window.YcControls = {
                        attachOnReadyQueue: [],
                        attach: function(element) {
                            this.attachOnReadyQueue.push(element);
                        },
                        onReadyQueue: [],
                        onReady: function(onReadyHandler) {
                            this.onReadyQueue.push(onReadyHandler);
                        },
                    };"
                }</script>
                {props.head_links}
                {props.head_scripts}
            </head>
            <body>
                {props.children}
                <script src={format!("/client/common.js?ts={}", *TS)}></script>
            </body>
        </html>
    }
}
