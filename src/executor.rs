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
    let num_threads = 3 as u16;
    for mut set_nm in start_set_nm..=end_set_nm {
        while end_set_nm - set_nm > num_threads {
            let mut trds = vec![];
            for i in set_nm..=(set_nm + num_threads) {
                let x = tokio::spawn(async move {
                    match downloader::download_set(i).await {
                        Ok(()) => {}
                        _ => {
                            println!("Finished downloading set {}", set_nm);
                        }
                    }
                });
                trds.push(x);
            }
            for trd in trds {
                trd.await.expect("TODO: panic message");
            }
            set_nm = set_nm + num_threads;
        }
    }
    Ok(())
}
