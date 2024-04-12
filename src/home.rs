use perseus::prelude::*;
use sycamore::prelude::*;

fn home_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
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
        div(style="height: 1000px")
        p { "I love you ❤️" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "OwO What is this?" }

        meta(name="description", content="Haii :3, I'm Baba, just your average nerdy femboy\nWelcome to my website :3")
        //TODO: maybe change the .perseus thing somehow
        link(href="/.perseus/static/assets/baba.css", rel="stylesheet")

        link(rel="preconnect", href="https://fonts.googleapis.com")
        link(rel="preconnect", href="https://fonts.gstatic.com", crossorigin=true)
        link(href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap", rel="stylesheet")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(home_page).head(head).build()
}
