#![allow(non_snake_case)]
use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_primitives::calendar::*;

fn main() {
    dioxus::launch(|| {
        rsx! {
            h1 { "Homeschool HQ" }
            body {
                Router::<Route> {}
            }
        }
    });
}

#[derive(serde::Deserialize)]
struct FirebaseResponse {
    idToken: String,
    email: String,
    refreshToken: String,
    expiresIn: String,
    localId: String,
    registered: bool,
}

#[derive(serde::Serialize)]
struct Payload {
    email: String,
    password: String,
    returnSecureToken: bool,
}

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home,
    #[route("/login")]
    Login,
    #[route("/planner")]
    Planner,
}

const FIREBASE_RESPONSE: GlobalSignal<Option<FirebaseResponse>> = Global::new(Option::default);

fn Home() -> Element {
    let onclick = |_| {
        use_navigator().push(Route::Login);
    };

    rsx! {
        h2 { "Home" }
        button { onclick, if FIREBASE_RESPONSE.read().is_some() { "Log out" } else { "Log in" } }
        br {}
        button { onclick: |_| { use_navigator().push(Route::Planner); }, "Planner" }
    }
}

fn Login() -> Element {
    let onclick = |_| {
        *FIREBASE_RESPONSE.write() = None;
        use_navigator().push(Route::Home);
    };
    let onsubmit = move |evt: FormEvent| async move {
        let payload = Payload {
            email: evt.values()["email"].as_value(),
            password: evt.values()["password"].as_value(),
            returnSecureToken: true,
        };
        let response = reqwest::Client::new()
            .post("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key=AIzaSyCvvAGxu3nyHzYkvlxv9_UqPKTPqOJqIdk")
            .json(&payload)
            .send()
            .await
            .unwrap();

        if response.status() == 200 {
            *FIREBASE_RESPONSE.write() = Some(response.json::<FirebaseResponse>().await.unwrap());
        }
        use_navigator().push(Route::Home);
    };

    rsx! {
        h2 { if FIREBASE_RESPONSE.read().is_some() { "Logout" } else { "Login" } }
        if FIREBASE_RESPONSE.read().is_some() {
            button { onclick, "Confirm logout" }
        } else {
            form { onsubmit,
                label { "Email" }
                br {}
                input { r#type: "text", id: "email", name: "email" }
                br {}
                label { "Password" }
                br {}
                input { r#type: "password", id: "password", name: "password" }
                br {}
                button { "Sign in" }
            }
        }
    }
}

pub fn Planner() -> Element {
    let date = chrono::Local::now();
    let mut selected_date = use_signal(|| None);
    let mut view_date = use_signal(|| CalendarDate::new(date.year(), date.month(), date.day()));

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        h2 { "Planner" }
        Calendar {
            selected_date: selected_date(),
            on_date_change: move |date| {
                selected_date.set(date);
            },
            view_date: view_date(),
            on_view_change: move |date| {
                view_date.set(date);
            },
            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton { class: "arrow left" }
                    CalendarMonthTitle {}
                    CalendarNextMonthButton { class: "arrow right" }
                }
            }
            CalendarGrid {}
        }
    }
}
