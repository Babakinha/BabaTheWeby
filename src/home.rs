use perseus::prelude::*;
use sycamore::prelude::*;

fn home_page<G: Html>(cx: Scope) -> View<G> {
    // Thingy
    const THINGYS: [&str; 3] = ["Dev", "Fluffy", "Noisy"];
    let thingy_index = create_rc_signal::<usize>(0);

    let thingy_index_clone = thingy_index.clone();
    create_effect(cx, move || {
        thingy_index_clone.track(); // Very important (bcs we use it on the client)

        // Only run on client
        #[cfg(client)]
        {
            use gloo::timers::callback::Timeout;

            // Roll the index around every 3.5s
            let thingy_index_clone = thingy_index_clone.clone();
            let to = Timeout::new(3500, move || {
                let new_thingy_index = *thingy_index_clone.get() + 1;
                if new_thingy_index >= THINGYS.len() {
                    thingy_index_clone.set(0);
                } else {
                    thingy_index_clone.set(new_thingy_index);
                }
            });
            on_cleanup(cx, || drop(to));
        }
    });

    // Mouse blob
    //TODO: Maybe use Pointer Events instead
    //TODO: Smooth this out
    let blob_x = create_rc_signal(-100);
    let blob_y = create_rc_signal(-100);
    blob_x.track();
    blob_y.track();
    #[cfg(client)]
    {
        let blob_x_clone = blob_x.clone();
        let blob_y_clone = blob_y.clone();
        use gloo::{events::EventListener, utils::window};
        EventListener::new(&window(), "mousemove", move |event| {
            use web_sys::wasm_bindgen::JsCast;

            let event: &web_sys::MouseEvent = event.unchecked_ref();
            blob_x_clone.set(event.x());
            blob_y_clone.set(event.y());
        }).forget(); //TODO: is this bad?
    }

    view! {cx,
        div(style=format!("background-color: pink; filter: blur(20px); width: 20px; height: 20px; position: fixed; left: {}px; top: {}px; z-index: -100;", blob_x.get(), blob_y.get()))

        div(style="margin: 10%;") {
            div(style="white-space: nowrap; text-align: left; font-size: 2rem") {
                h1 { "Haii :3" br {} " I'm Baba The " (THINGYS[*thingy_index.get()]) }
            }
        }
    }
    // view! { cx,
    //     h1 { "Hewwo wowld :3" }
    //     p {
    //         "Haii :3, I'm Baba, just your average nerdy "
    //         i { "(social anxiety and depression filled)" }
    //         " Femboi! ^^"
    //     }
    //     p {
    //         "I didnt really think to what i should put here yet... "
    //         i { "(and tbh im just kinda lazy to do it)" }
    //     }
    //     p {
    //         "but... i will prob put some cool stuff here... "
    //         i { "someday..." }
    //     }
    //     br {}
    //     p { "so uhm... i guess thats it, hope u have a nice dayy :3" }
    //     div(style="height: 1000px")
    //     p { "I love you ❤️" }
    // }
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
