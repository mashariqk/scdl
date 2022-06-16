use crate::configuration::get_configuration;
use crate::downloader;
use error_chain::error_chain;
use futures::executor::block_on;
use std::thread;

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
        while end_set_nm - set_nm < num_threads {
            let mut threads = vec![];
            for i in set_nm..=(set_nm + num_threads) {
                let x = thread::spawn(move || match block_on(downloader::download_set(i)) {
                    Ok(()) => {}
                    _ => {
                        println!("Finished downloading set {}", set_nm);
                    }
                });
                threads.push(x);
            }
            set_nm = set_nm + num_threads;
        }
    }
    Ok(())
}
