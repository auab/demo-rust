#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::{h1, a}};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}



pub fn Article(cx: Scope) -> Element {
    let lorem: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    cx.render(rsx!(
        article{
            class: "card",
            div{
                class: "article-title__div",
                h3{
                    "Título texto"
                }
            }
            
                div{
                    class: "article-text__div",
                    "{lorem}",
                    a{
                        href: "https://google.com",
                        "[...]"
                    }
                }
        }
))
}


fn App(cx: Scope) -> Element {

    
    cx.render(rsx!(
        h1{
            class: "main-title",
            "Blog Teste",
        },
        hr{},
        Article{},
        Article{},





))}

