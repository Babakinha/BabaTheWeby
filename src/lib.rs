pub mod app;
pub mod home;

pub const EXPORT_PATH: Option<&'static str> = option_env!("EXPORT_PATH");
pub const LEPTOS_SERVE_MODE: Option<&'static str> = option_env!("LEPTOS_SERVE_MODE");

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
