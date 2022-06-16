use crate::configuration::get_configuration;
use crate::downloader;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        DownErr(downloader::Error);
    }
}

pub async fn run() -> Result<()> {
    let config = get_configuration().expect("Config failure!");
    let set_nm = config.set_nm;
    downloader::download_set(set_nm).await?;
    Ok(())
}
