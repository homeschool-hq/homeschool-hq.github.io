#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        rsx! {
            body {
                Router::<Route> {}
            }
        }
    });
}

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home,
    #[route("/login")]
    Login,
}

fn Home() -> Element {
    rsx! {
        h1 { "Homeschool HQ" }
        Link { to: Route::Login {}, class: "nav-btn", "Login" }
    }
}

fn Login() -> Element {
    let onsubmit = move |evt: FormEvent| async move {};

    rsx! {
        h1 { "Login" }
        form { onsubmit,
            label { "Username" }
            br {}
            input { r#type: "text", id: "username", name: "username" }
            br {}
            label { "Password" }
            br {}
            input { r#type: "password", id: "password", name: "password" }
            br {}
            button { "Sign in" }
        }
    }
}
