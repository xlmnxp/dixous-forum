use dioxus::{document::Title, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{components::{InputField, InputType}, Route};

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn ThreadsCreate() -> Element {
    let title: Signal<String> = use_signal(|| "".into());
    let body: Signal<String> = use_signal(|| "".into());

    rsx! {
        Title {
            "Dixous Forum - Create Thread"
        }
        div {
            class: "p-10",
            id: "thread",

            InputField {
                label: "Title",
                full_width: true,
                signal: title
            }

            InputField {
                label: "Body",
                full_width: true,
                input_type: InputType::Textarea,
                signal: body
            }

            button {
                class: "bg-white p-2 text-black font-bold cursor-pointer active:opacity-90 hover:opacity-95",
                onclick: move |_| {
                    async move {
                        if let Ok(thread_id) = create_thread(title(), body()).await {
                            navigator().push(Route::ThreadsView { id: thread_id });
                        }
                    }
                },
                "Submit"
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
async fn create_thread(title: String, body: String) -> Result<i32, ServerFnError> {
    use crate::database::models::*;
    use crate::database::sqlite::establish_connection;
    use crate::schema::threads;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    let new_thread = NewThread {
        title: &title,
        body: &body
    };

    let thread = diesel::insert_into(threads::table)
        .values(&new_thread)
        .returning(Thread::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    Ok(thread.id)
}
