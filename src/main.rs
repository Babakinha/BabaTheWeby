use std::process::exit;

#[cfg(feature = "export")]
#[tokio::main]
async fn main() {
    use baba_pet::{app::*, EXPORT_PATH, LEPTOS_SERVE_MODE};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;

    let leptos_serve_mode = LEPTOS_SERVE_MODE.unwrap_or("SERVE");
    match leptos_serve_mode {
        "BUILD" => {
            if let Some(path) = EXPORT_PATH {
                let dist = leptos_options.site_root;
                std::fs::rename(format!("{dist}/{path}.html"), format!("{dist}/index.html"))
                    .expect("Unable to rename export");
            }
        }

        "SERVE" => {
            let app = axum::Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell))
                .with_state(leptos_options);

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }

        _ => {
            eprintln!("Not a valid LEPTOS_SERVE_MODE");
            exit(1);
        }
    }
}

#[cfg(not(feature = "export"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
//#[perseus::main(perseus_axum::dflt_server)]
//pub fn main<G: Html>() -> PerseusApp<G> {
//    PerseusApp::new()
//        .template(home::get_template())
//        .error_views(ErrorViews::unlocalized_development_default())
//        //TODO: Better localization
//        .index_view_str(INDEX_VIEW)
//        .static_dir("./static")
//        // Public stuff
//        //TODO: Automate this?
//        .static_alias("/favicon.ico", "./public/favicon.ico")
//        .static_alias("/robots.txt", "./public/robots.txt")
//        // Github Pages stuff
//        // Idk if this is needed for gh pages to work but... why not keep it
//        .static_alias("/CNAME", "./CNAME")
//        .static_alias("/.nojekyll", "./.nojekyll")
//}

//// Just to add lang="en" :3
//static INDEX_VIEW: &str = r#"
//<html lang="en">
//    <head>
//        <meta charset="UTF-8" />
//        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
//    </head>
//    <body>
//        <div id="root"></div>
//    </body>
//</html>"#;
