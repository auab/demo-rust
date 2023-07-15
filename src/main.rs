#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::{h1, a}};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}


#[derive(Props)]
pub struct ArticleFieldsProps<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub text: &'a str,
}

pub fn Article<'a>(cx: Scope<'a, ArticleFieldsProps<'a>>) -> Element {
        cx.render(rsx!(
        article{
            class: "article-card",
            div{
                class: "article-title__div",
                h3{"{cx.props.title}"}
            }
                div{
                    class: "article-text__div",
                    "{cx.props.text}",
                    a{
                        href: "https://google.com",
                        "[...]"
                    }
                }
                div{
                    class: "article-author__div",
                    "{cx.props.author}"
                }
        }
))
}


fn App(cx: Scope) -> Element {    
    let lorem: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    cx.render(rsx!(
        h1{
            class: "main-title",
            "Blog Teste",
        },
        hr{},
        div{
            class: "articles-div",
            Article{title:"A New Technology is emerging",author:"Alexandre",text:"{lorem}"},
        }

        // Article{title:"b"},
))}

