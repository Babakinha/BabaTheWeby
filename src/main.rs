mod home;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(home::get_template())
        .error_views(ErrorViews::unlocalized_development_default())
        .static_dir("./static")
        .static_alias("/favicon.ico", "./static/favicon.ico")

        // Idk if this is needed for gh pages to work but... why not keep it
        .static_alias("/CNAME", "./CNAME")
}
