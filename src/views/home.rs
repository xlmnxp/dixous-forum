use dioxus::{document::Title, prelude::*};
use serde::{Deserialize, Serialize};

use crate::Route;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Home() -> Element {
    let get_threads = use_resource(get_threads);
    rsx! {
        Title {
            "Dixous Forum - Threads"
        }
        div {
            class: "p-10",
            id: "threads",
            div {
                class: "flex justify-between",
                h2 {
                    class: "font-bold text-white text-2xl border-b-2 mb-2 grow",
                    "Threads"
                }

                Link {
                    to: Route::ThreadsCreate {  },
                    button {
                        class: "bg-white ml-2 mb-2 p-2 text-black font-bold cursor-pointer active:opacity-90 hover:opacity-95",
                        "Create Thread"
                    }
                }
            }
            match &*get_threads.read() {
                Some(Ok(all_threads)) => {
                    rsx! {
                        for thread in all_threads.iter() {
                            Link {
                                key: thead.id,
                                class: "text-white active:opacity-75 hover:opacity-85",
                                to: Route::ThreadsView {
                                    id: thread.id
                                },
                                h2 { "{thread.id}. {thread.title}" }
                            }
                        }
                    }
                },
                    _ => rsx!( h1 {
                        class: "text-white text-center",
                        "Loading Threads..."
                    } )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OutThreads {
    pub id: i32,
    pub title: String,
    pub body: String,
}

/// Echo the user input on the server.
#[server]
async fn get_threads() -> Result<Vec<OutThreads>, ServerFnError> {
    use crate::database::models::*;
    use crate::database::sqlite::establish_connection;
    use crate::schema::threads::dsl::threads;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    let all_threads: Vec<crate::database::models::Thread> = threads
        .select(crate::database::models::Thread::as_select())
        .load(connection)
        .expect("Error loading threads");

    Ok(all_threads
        .into_iter()
        .map(|thread| OutThreads {
            id: thread.id,
            title: thread.title,
            body: thread.body,
        })
        .collect())
}
