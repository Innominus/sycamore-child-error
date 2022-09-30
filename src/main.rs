use examples::sub_component::{DrawerOverlay, TitleBar};
use sycamore::prelude::*;

mod examples;

#[component]
fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        // Component with sub item
        h1 {"Sub component"}
        DrawerOverlay() {
            div(class="w-full h-full flex flex-col items-center") {
                TitleBar
                div {
                    "Home!"
                }
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            App {}
        }
    });
}
