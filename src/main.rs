#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use sycamore::prelude::*;

mod rust_nairobi_logo;
pub use rust_nairobi_logo::*;

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
        div(class="palette-01 frow  row-center height-100vhmin") {
            div(class="frow col-md-1-2") {
                div(class="svg-50 "){
                    (svg_logo(cx))
                }
            }
            div(class="frow col-md-1-2") {
                div(class="frow direction-column row-center width-100") {
                    h3 { "Hello, Rusty MiniBank here!" }
                    h4 { "Make by Rust-Nairobi Developers for Rusteceans" }
#[component]
fn SignUp<G: Html>(cx: Scope) -> View<G> {
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
