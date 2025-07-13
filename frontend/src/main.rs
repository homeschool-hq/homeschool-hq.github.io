#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| rsx! {
        body {
            Router::<Route> {}
        }
    });
}

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home,
}

fn Home() -> Element {
    rsx! {
        h1 { "Homeschool HQ" }
    }
}
