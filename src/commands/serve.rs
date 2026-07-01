use anyhow::{Result, bail};
use std::path::PathBuf;

pub fn serve_repo(path: PathBuf, port: u16) -> Result<()> {
    if !path.exists() {
        bail!("Path {:?} does not exist.", path);
    }

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Serving directory {:?} on http://{}", path, addr);

    rouille::start_server(addr, move |request| {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            response
        } else {
            rouille::Response::html("404 Not Found").with_status_code(404)
        }
    });
}
