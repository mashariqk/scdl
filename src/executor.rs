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
    let mut counter = start_set_nm;
    while end_set_nm - counter >= num_threads {
        let mut trds = vec![];
        for i in counter..(counter + num_threads) {
            let x = tokio::spawn(async move {
                match downloader::download_set(i).await {
                    Ok(()) => {}
                    _ => {
                        println!("Finished downloading set {}", i);
                    }
                }
            });
            trds.push(x);
        }
        for trd in trds {
            trd.await.expect("TODO: panic message");
        }
        counter += num_threads;
    }
    if counter <= end_set_nm {
        for i in counter..=end_set_nm {
            match downloader::download_set(i).await {
                Ok(()) => {}
                _ => {
                    println!("Finished downloading set {}", i);
                }
            }
        }
    }
    Ok(())
}
