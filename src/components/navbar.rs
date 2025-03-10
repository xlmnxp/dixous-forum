use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "bg-gray-700 w-full h-full absolute",
            div {
                class: "p-2 bg-gray-600 flex justify-between items-center  pl-10 pr-10",
                id: "navbar",
                h1 {
                    class: "inline-block text-white",
                    "Dixous Forum"
                }
                Link {
                    to: Route::Home {},
                    class: "bg-white p-2 rounded-xs m-1 inline-block active:opacity-90 hover:opacity-95",
                    "Home"
                }
            }
    
            Outlet::<Route> {}
        }
    }
}
