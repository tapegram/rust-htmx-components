# rust-htmx-components

## Todos

How do we build the tailwind styles even though we are a dependency of another project?

I think we have to

1) expose the a route with the mounted assets from this library. This route is pointed to by html_layout. We will need to keep the CSS build step here

2) the user of this library will need to expose that assets endpoint.

3) If they want to use their own tailwind styles, they will have to add their own tailwind integration and expose a separate assets route for themselves.

