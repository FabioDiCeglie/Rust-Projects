use leptos::*;
use leptos_meta::*;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Similar to useState in React: `conversation` is the state, and `set_conversation` is the function to update it
    let(conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        // TODO converse
        async move {}
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ai-chat.css"/>

        <Title text="Rust Chat AI"/>
        <ChatArea conversation />
        <TypeArea send />
    }
}