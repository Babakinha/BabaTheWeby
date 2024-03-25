use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let text = use_state(cx, || "...".to_string());

    cx.render(rsx! {
        div {
            h1 { "Hewwo wowld :3" }
            p {
                "Haii :3, I'm Baba, just your average nerdy "
                i { "(social anxiety and depression filled)" }
                " Femboi! ^^"
            }
            p {
                "I didnt really think to what i should put here yet... "
                i { "(and tbh im just kinda lazy to do it)" }
            }
            p {
                "but... i will prob put some cool stuff here... "
                i { "someday..." }
            }
            br {}
            p { "so uhm... i guess thats it, hope u have a nice dayy :3" }
            div { style: "height: 1000px" }
            p { "I love you ❤️" }
        }
    })
}
