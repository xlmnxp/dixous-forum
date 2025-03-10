use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum InputType {
    Input,
    Textarea,
}

#[component]
pub fn InputField(
    label: String,
    full_width: Option<bool>,
    input_type: Option<InputType>,
    signal: Option<Signal<String>>
) -> Element {
    rsx! {
        div {
            class: "mb-4",
            div {
                class: "text-white font-bold mb-2",
                "{label}"
            }
            {
                match input_type {
                    Some(InputType::Textarea) => rsx! {
                        textarea {
                            class: format!("bg-gray-600 border-b-2 border-white text-white p-2 outline-0 {}", match full_width {
                                Some(true) => "w-full",
                                _ => ""
                            }),

                            oninput: move |event| {
                                if let Some(mut signal) = signal {
                                    signal.set(event.value());
                                }
                            }
                        }
                    },
                    _ => rsx! {
                        input {
                            class: format!("bg-gray-600 border-b-2 border-white text-white p-2 outline-0 {}", match full_width {
                                Some(true) => "w-full",
                                _ => ""
                            }),

                            oninput: move |event| {
                                if let Some(mut signal) = signal {
                                    signal.set(event.value());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
