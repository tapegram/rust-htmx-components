use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::post,
    Router,
};
use rscx::{component, html, props};

use components::server::form::{Button, FileInput, Label};

pub fn file_input_routes() -> Router {
    Router::new().route("/", post(upload_file))
}

// ### Route Handlers ###

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    println!("Uploading file!");
    println!("Multipart: {:?}", multipart);

    let mut content: String = String::new();

    // from: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        content.push_str(format!("Length of `{}` is {} bytes", name, data.len()).as_str());
    }

    Html(html! {
        <div class="bg-white p-10 border">
            <p>"✅ File uploaded!"</p>
            <h1 class="mt-4 font-bold">Additional Info</h1>
            <code>{content}</code>
        </div>
    })
}

// ### Components ###

#[component]
pub fn FileInputPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">FileInput Playground</h2>
            <div class="col-span-full">
                <form id="form"
                    hx-encoding="multipart/form-data"
                    hx-post="/playground/file-input"
                    hx-target="#form"
                >
                    <Label for_input="cover-photo">
                        You Favorite Photo
                    </Label>
                    <FileInput
                        id="fav-photo-file"
                        name="fav-photo-file"
                        file_hint_message="PNG, JPG, GIF up to 10MB"
                        accept="image/png, image/jpeg, image/gif"
                    />
                    <Button class="mt-4" kind="submit">
                        Submit
                    </Button>
                </form>
            </div>
        </section>
    }
}
