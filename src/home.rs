use std::{cell::RefCell, rc::Rc};

use perseus::prelude::*;
use sycamore::prelude::*;

fn home_page<G: Html>(cx: Scope) -> View<G> {
    // Thingy
    const THINGYS: [&str; 3] = ["Dev", "Fluffy", "Noisy"];
    const THINGYS_CSS_COLOR: [&str; 3] = ["--baba-color", "--femboi-color", "--blue-color"];
    const THINGYS_IMAGE: [&str; 3] = ["buggy.svg", "owo.svg", "noisy.svg"];
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
    //TODO: Make the blob shape not just a circle
    let blob_pos = Rc::new(RefCell::new((-100, -100)));
    #[cfg(client)]
    {
        use gloo::{events::EventListener, utils::window};
        use web_sys::wasm_bindgen::JsCast;

        //TODO: is forgetting bad?
        //TODO: is it ok to have lots of event listeneers like this?
        let blob_pos_clone = blob_pos.clone();
        EventListener::new(&window(), "pointermove", move |event| {
            let event: &web_sys::PointerEvent = event.unchecked_ref();
            blob_pos_clone.replace((event.client_x(), event.client_y()));
        })
        .forget();
        

        let blob_pos_clone = blob_pos.clone();
        EventListener::new(&window(), "touchmove", move |event| {
            let event: &web_sys::TouchEvent = event.unchecked_ref();
            if let Some(touch) = event.changed_touches().item(0) {
                blob_pos_clone.replace((touch.client_x(), touch.client_y()));
            }
        })
        .forget();

        let blob_pos_clone = blob_pos.clone();
        EventListener::new(&window(), "touchstart", move |event| {
            let event: &web_sys::TouchEvent = event.unchecked_ref();
            if let Some(touch) = event.changed_touches().item(0) {
                blob_pos_clone.replace((touch.client_x(), touch.client_y()));
            }
        })
        .forget();
    }

    let blob_smooth_pos = create_rc_signal((-100.0, -100.0));
    let blob_velocity = Rc::new(RefCell::new((0.0, 0.0)));
    let blob_acceleration = 0.15;
    let blob_max_speed = 100.0;

    let blob_smooth_pos_clone = blob_smooth_pos.clone();
    let blob_velocity_clone = blob_velocity.clone();
    //TODO: Use requestAnimationFrame
    create_effect(cx, move || {
        blob_smooth_pos_clone.track(); // Very important (bcs we use it on the client)

        // Only run on client
        #[cfg(client)]
        {
            use gloo::timers::callback::Timeout;

            // Roughly 60fps
            let blob_smooth_pos_clone = blob_smooth_pos_clone.clone();
            let blob_velocity_clone = blob_velocity_clone.clone();
            let blob_pos = blob_pos.borrow();
            let mouse_x = blob_pos.0 as f64;
            let mouse_y = blob_pos.1 as f64;
            let to = Timeout::new(16, move || {
                let smooth_pos = blob_smooth_pos_clone.get();
                let dx = mouse_x - smooth_pos.0 as f64;
                let dy = mouse_y - smooth_pos.1 as f64;

                blob_velocity_clone.replace_with(|_velocity| {
                    (
                        (dx * blob_acceleration).clamp(-blob_max_speed, blob_max_speed),
                        (dy * blob_acceleration).clamp(-blob_max_speed, blob_max_speed),
                    )
                });

                let mut pos = *blob_smooth_pos_clone.get().clone();
                pos.0 += blob_velocity_clone.borrow().0;
                pos.1 += blob_velocity_clone.borrow().1;
                blob_smooth_pos_clone.set(pos);
            });
            on_cleanup(cx, || drop(to));
        }
    });

    // wanna see smthn dumb?
    let thingy_index_clone = thingy_index.clone();
    let thingy_index_clone_clone = thingy_index.clone();

    view! {cx,
        // Blob
        div(class="blob", style=format!("left: {}px; top: {}px; --interest-color: var({});", blob_smooth_pos.get().0, blob_smooth_pos.get().1, THINGYS_CSS_COLOR[*thingy_index_clone.get()]))

        div(class="intro-box") {
            div(class="home-title") {
                h1 { "Hewwo world :3" br {} "I'm Baba " wbr {} span(style="display: inline-block;") { "The " (THINGYS[*thingy_index.get()]) } }
            }
            div(class="home-description") {
                p {
                    "Your average nerdy "
                        i(style="display: inline-block; white-space: nowrap;") { "(social anxiety and depression filled)" }
                    " Femboi! :3"
                }
                p {
                    "I didnt really think to what i should put here yet... "
                        i(style="display: inline-block; white-space: nowrap;") { "(and tbh im just kinda lazy to do it)" }
                }
                p {
                    "but... i will prob put some cool stuff here... "
                        i { "someday..." }
                }
            }

            img(src=format!("/.perseus/static/assets/{}", THINGYS_IMAGE[*thingy_index_clone_clone.get()]), width="400px", height="400px")
        }

        div(class="line", style="margin-top: 200px")

        div(class="projects-box") {
            div(class="simple-title") {
                h1 { "Projectzz" }
            }
            div(class="simple-description") {
                p { "Check out my projectzz >:3" }
                p { "These are not all of them, but i think they are pretty cool" }
                p(style="color: #666a") { "//TODO: This is very unfinished -.-" }
            }

            div(class="projects") {
                //TODO: Components?
                a(class="project-card", href="/babiano") {
                    img(src="/.perseus/static/assets/projects/babiano.svg", width="400px", height="400px")
                }
            }

        }



        div(style="height: 1000px")

        p(class="sticky-love") { "I love you ❤️" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "OwO What is this?" }

        meta(name="description", content="Haii :3, I'm Baba, just your average nerdy femboy\nWelcome to my website :3")

        link(rel="preconnect", href="https://fonts.googleapis.com")
        link(rel="preconnect", href="https://fonts.gstatic.com", crossorigin=true)
        link(rel="preload", as="style", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        link(rel="stylesheet", media="print", onload="this.media='all'", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        noscript {
            link(rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        }
        
        //TODO: maybe change the .perseus thing somehow
        link(href="/.perseus/static/assets/baba.css", rel="stylesheet")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(home_page).head(head).build()
}
