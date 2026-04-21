use leptos::prelude::*;

use leptos_meta::{Link, Meta, Title};

use leptos_use::{use_event_listener, use_raf_fn, utils::Pausable};

#[component]
pub fn HomePage() -> impl IntoView {
    // Thingy
    const THINGYS: [&str; 3] = ["Dev", "Fluffy", "Noisy"];
    const THINGYS_IMAGE: [&str; 3] = ["buggy.svg", "owo.svg", "noisy.svg"];
    const THINGYS_CSS_COLOR: [&str; 3] = ["--baba-color", "--femboi-color", "--blue-color"];
    const THINGY_INTERVAL_MS: u32 = 3500;

    // Signals
    let thingy_index = RwSignal::new(0);
    let mouse_pos = RwSignal::new((-100, -100));
    let blob_smooth_pos = RwSignal::new((-100.0, -100.0));
    let blob_velocity = RwSignal::new((0.0, 0.0));

    // Roll the index around every 3.5s
    #[cfg(feature = "hydrate")]
    {
        use gloo_timers::callback::Interval;
        let timer = Interval::new(THINGY_INTERVAL_MS as u32, move || {
            thingy_index.update(|i| *i = (*i + 1) % THINGYS.len());
        });
        timer.forget();
        // on_cleanup(move || drop(timer));
    }

    // Set mouse position
    #[cfg(feature = "hydrate")]
    {
        use leptos::ev;

        let _ = use_event_listener(window(), ev::pointermove, move |ev| {
            mouse_pos.set((ev.client_x(), ev.client_y()));
        });

        let touch_handler = move |ev: web_sys::TouchEvent| {
            if let Some(touch) = ev.changed_touches().item(0) {
                mouse_pos.set((touch.client_x(), touch.client_y()));
            }
        };
        let _ = use_event_listener(window(), ev::touchmove, touch_handler.clone());
        let _ = use_event_listener(window(), ev::touchstart, touch_handler);

        let Pausable { pause, resume, .. } = use_raf_fn(move |_| {
            let (mx, my) = mouse_pos.get();
            let (sx, sy) = blob_smooth_pos.get();

            let dx = mx as f64 - sx;
            let dy = my as f64 - sy;

            let acc = 0.15;
            let max_speed = 100.0;

            let vx = (dx * acc).clamp(-max_speed, max_speed);
            let vy = (dy * acc).clamp(-max_speed, max_speed);

            blob_velocity.set((vx, vy));
            blob_smooth_pos.set((sx + vx, sy + vy));
        });

        resume();
        on_cleanup(move || pause());
    }

    view! {
        <Title text="OwO What is this?" />
        <Meta name="description" content="Haii :3, I'm Baba, just your average nerdy femboy\nWelcome to my website :3" />
        <Link rel="stylesheet" href="/assets/baba.css" />
        // <Link rel="preconnect" href="https://fonts.googleapis.com" />
        // <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=true />
        // <Link rel="preload" as="style" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        // <Link rel="stylesheet" media="print" onload="this.media='all'" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        // // noscript {
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        // // }


        // The Blob
        <div
            class="blob"
            style=move || format!(
                "left: {}px; top: {}px; --interest-color: var({});",
                blob_smooth_pos.get().0,
                blob_smooth_pos.get().1,
                THINGYS_CSS_COLOR[thingy_index.get()]
            )
        />

        <div class="intro-box">
            <div class="intro-text">
                <h1>
                    "Hewwo world :3" <br />
                    "I'm Baba " <wbr />
                    <span style="display: inline-block;">
                        "The " {move || THINGYS[thingy_index.get()]}
                    </span>
                </h1>
                <p>
                    "Your average nerdy "
                    <i style="display: inline-block; white-space: nowrap;">"(social anxiety and depression filled)"</i>
                    " Femboi! :3"
                </p>
                <p>
                    "I didnt really think to what i should put here yet... "
                    <i style="display: inline-block; white-space: nowrap;">"(and tbh im just kinda lazy to do it)"</i>
                </p>
                <p>"but... i will prob put some cool stuff here... " <i>"someday..."</i></p>
            </div>

            <div class="intro-image">
            <img
                src=move || format!("/assets/{}", THINGYS_IMAGE[thingy_index.get()])
                width="400px"
                height="400px"
            />
            </div>
        </div>

        <div class="line" style="margin-top: 200px" />

        <div class="projects-box">
            <div class="simple-title"><h1>"Projectzz"</h1></div>
            <div class="simple-description">
                <p>"Check out my projectzz >:3"</p>
                <p>"These are not all of them, but i think they are pretty cool"</p>
                <p style="color: #666a">"//TODO: This is very unfinished -.-"</p>
            </div>

            <div class="projects">
                <a class="project-card" href="/babiano" rel="external">
                    <img src="/assets/projects/babiano.svg" width="400px" height="400px" />
                </a>
            </div>
        </div>

        <div style="height: 1000px" />
        <p class="sticky-love">"I love you ❤️"</p>
    }
}
