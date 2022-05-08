#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use sycamore::prelude::*;
use sycamore_router::{Route,Router, HistoryIntegration};

mod rust_nairobi_logo;
pub use rust_nairobi_logo::*;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/signup")]
    SignUp,
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            App()
        }
    });
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="app") {
                        (match route.get().as_ref() {
                            AppRoutes::Index => view! { cx,
                                Index()
                            },
                            AppRoutes::SignUp => view! { cx,
                                SignUp()
                            },
                            AppRoutes::NotFound => view! { cx,
                                NotFound()
                            },
                        })
                    }
                }
            }
        }
    }
}

#[component]
fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="frow row-center h-100vh bg-p") {
            div(class="frow col-md-1-2") {
                div(class="w-50"){
                    (svg_logo(cx))
                }
            }
            div(class="frow col-md-1-2") {
                div(class="frow direction-column row-center w-100") {
                    h3 { "Hello, Rusty MiniBank here!" }
                    h4 { "Make by Rust-Nairobi Developers for Rusteceans" }
                    a(href="/signup", type="button", class="bg-s my-10") { "See demo" }
                }
            }
        }
    }
}

#[component]
fn SignUp<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="frow  row-center h-100vh") {
            div(class="frow centered-column col-md-1-3 bg-p minh-100vh") {
                h1(class="fg-w") {
                   a(href="/", class="fg-w") { "Mini Bank" }
                }
                div(class="w-50"){
                    (svg_logo(cx))
                }
            }
            div(class="frow row-center col-md-2-3") {
                SignUpForm()
            }
        }
    }
}

#[component]
fn SignUpForm<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        form(class="form") {
            h1 { "Create an account"}
            label {
                "Username"
                input(
                    class="h-40px",
                    type="text",
                    placeholder="john_doe"
                )
            }
            label {
                "Email"
                input(
                    class="h-40px",
                    type="email",
                    placeholder="johndoe@example.com"
                )
            }
            label {
                "Password"
                input(
                    class="h-40px",
                    type="password",
                    placeholder="complicated"
                )
            }
            label(class="row-start") {
                input(
                    type="checkbox",
                    value="toc"
                )
                "I agree with the "
                a {
                    "terms of service"
                }
            }
            button(class="h-40px") {
                "Create account"
            }
        }
    }
}

#[component]
fn NotFound<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 { "Error 404" }
        h4 { "Page not found" }
    }
}
