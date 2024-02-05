use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct YcControlProps {
    #[builder(setter(into), default)]
    control: String,

    #[builder(default)]
    children: String,
}

#[component]
pub fn YcControl(props: YcControlProps) -> String {
    html! {
        <HtmlElement
            attrs=spread_attrs!(props).set("data-yc-control", props.control)
        >
            {props.children}
            <script>"YcControls.attach(document.currentScript.parentElement);"</script>
        </HtmlElement>
    }
}

#[html_element]
pub struct ToggleProps {
    #[builder(default)]
    children: String,
}

#[component]
pub fn Toggle(props: ToggleProps) -> String {
    html! {
        <YcControl
            control="toggle"
            attrs=spread_attrs!(props)
        >
            {props.children}
        </YcControl>
    }
}

#[props]
pub struct YcControlJsApiProps {
    #[builder(setter(into))]
    call: String,
}

#[component]
pub fn YcControlJsApi(props: YcControlJsApiProps) -> String {
    html! {
        <script>
            {format!(
                r#"
                    (function(callerScript) {{
                        YcControls.onReady(function() {{
                            YcControls.{};
                        }});
                    }}(document.currentScript));
                "#,
                props.call,
            )}
        </script>
    }
}
