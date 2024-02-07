# rust-htmx-components

This is a POC library which provides some JS enriched HTMX components so you can build nifty webapps with common things such as transitions, modals, and notifications while writing little to no javascript.

Give it a try and let us know your experience!

![Playground Recording](docs/playground-recording.mov)

## Summary

This lib provides a collection of [RSCX](https://github.com/pitasi/rscx)-based rust components with behavior provided by [htmx-glue](https://github.com/tapegram/htmx-glue) so you can do [HTMX](https://htmx.org/) in style almost purely in Rust!

The components are pre-styled with tailwind.

## Usage

See examples/playground for a simple example app with a playground page you can test out the components in.

Most of the "magic" lives in `html_layout`

```html
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
        <script src="https://unpkg.com/htmx-glue/out/common.js"></script>
    </body>
</html>
```

We include tailwind via the CDN. This is not ideal by any means and we will be back with something better in the near future!

We wire up the JS from htmx-glue next (and the actual JS from the lib is in a script tag further down). We init and attach our `YcControls` to the window to bootstrap all of the htmx-glue controls here!

In order to use this, take a look at `page_layout` in examples/playground.
        
```html
<HtmlLayout
    head_scripts={
        html! {
            // Use unminified source for debugging.
            // <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.js"></script>
            <script
                src="https://unpkg.com/htmx.org@1.9.9"
                integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX"
                crossorigin="anonymous"
            ></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/response-targets.js"></script>
            <script>{
                r#"
                htmx.on("htmx:sendError", function() {
                    YcControls.showErrorNotification("Network Error!");
                });                

                htmx.on("htmx:responseError", function(error) {
                    YcControls.showErrorNotification(
                        error.detail.xhr.responseText || "Unknown error"
                    );
                });

                document.addEventListener("htmx:confirm", function(e) {
                    if (!e.target.hasAttribute("hx-confirm")) return true;            
                    e.preventDefault();
                    YcControls.confirm({
                        title: e.target.getAttribute("hx-confirm"),
                        message: e.target.dataset.confirmMessage,
                        actionConfirmed: function() {
                            e.detail.issueRequest(true);
                        }
                    });
                });
                "#
            }</script>
        }
    }
>
    <AppShell header=props.header>
        <main hx-ext="loading-states">
            {props.children}
        </main>
    </AppShell>
    <ModalProxy />
    <div hx-history-elt>
        <NotificationLiveRegion />
        <ModalLiveRegion />
    </div>
</HtmlLayout>
```

This is where we add HTMX and do some final wiring. Including hooking up with the htmx error events so we can display them as notifcations and providing notification and modal live regions, for those components to work properly.

## Todos/Disclaimers

At the moment, we are using Tailwind from a CDN, which is strongly discouraged from production use. So look forward to that!

## Acknowledgements

This library is directly derived from work [Paul Bouzakis](https://github.com/pbouzakis) did on a project we were working on, where we were experimenting with building a web app with htmx and rust.

All I ([Tavish Pegram](https://github.com/tapegram)) did was pull the pieces out into reusable libraries so we could share it out.


