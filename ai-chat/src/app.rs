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

    // Create an effect that triggers when there's a new input in the `send` action.
    // This effect adds a "typing" message to the conversation when input is detected.
    create_effect(move |_| {
        if let Some(_) = send.input().get(){
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c|{
                c.messages.push(model_message);
            });
        }
    });
    
    // Create an effect that triggers when a new value is available from the `send` action.
    // This effect updates the last message in the conversation with the response received.
    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get(){
            set_conversation.update(move |c|{
                c.messages.last_mut().unwrap().text = response
            });
        }
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