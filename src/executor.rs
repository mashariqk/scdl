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
    let start_set_nm = config.start_set_nm;
    let end_set_nm = config.end_set_nm;
    for set_nm in start_set_nm..=end_set_nm {
        match downloader::download_set(set_nm).await {
            Ok(()) => {}
            _ => {
                println!("Finished downloading set {}", set_nm);
            }
        }
    }
    Ok(())
}
