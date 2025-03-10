use dioxus::{document::Title, prelude::*};
use serde::{Deserialize, Serialize};

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn ThreadsView(id: i32) -> Element {
    let get_thread = use_resource(move || get_thread_by_id(id));

    rsx! {
        div {
            class: "p-10",
            id: "thread",
            match &*get_thread.read() {
                Some(Ok(thread)) => {
                    rsx! {
                        Title {
                            "Dixous Forum - {thread.title}"
                        }

                        h2 {
                            class: "font-bold text-white text-2xl border-b-2 mb-2",
                            "{thread.id}. {thread.title}"
                        }
                        p {
                            class: "bg-gray-200 p-2", 
                            "{thread.body}"
                        }
                    }
                },
                    _ => rsx!( h1 { "Loading Threads List"} )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OutThread {
    pub id: i32,
    pub title: String,
    pub body: String,
}

/// Echo the user input on the server.
#[server]
async fn get_thread_by_id(id: i32) -> Result<OutThread, ServerFnError> {
    use crate::database::models::*;
    use crate::database::sqlite::establish_connection;
    use crate::schema::threads::dsl::threads;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    let all_threads = threads
        .find(id)
        .select(crate::database::models::Thread::as_select());

    let thread = all_threads.first(connection).expect("Error loading threads");

    Ok(OutThread {
        id: thread.id,
        title: thread.title.clone(),
        body: thread.body.clone(),
    })
}
