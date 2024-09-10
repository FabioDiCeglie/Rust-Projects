use leptos::*;
use leptos_meta::*;

use crate::{api::converse, model::conversation::{Conversation, Message}};

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

        // Function implemented in the server side logic but we invoke this in the client side
        // the client is automatically generating code to make an HTTP request pointing to the Back-end API expose
        // by actix-web and all is happening behinde the scene, so everything is abstracted from us and this is very cool!!
        converse(conversation.get())
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