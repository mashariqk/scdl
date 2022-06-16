use crate::configuration::get_configuration;
use crate::downloader;
use error_chain::error_chain;
use std::fs::OpenOptions;
use std::io::Write;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        DownErr(downloader::Error);
    }
}

pub async fn run() -> Result<()> {
    let config = get_configuration().expect("Config failure!");
    let url1 = config.get_url_to_append_set();
    let mod_nm = config.mod_nm;
    let set_nm = config.set_nm;
    let m_prepend = &mod_nm[..2];
    let m_append = config.mod_append;
    let joiner = config.joiner;
    let client = reqwest::Client::new();
    let parent_dir_nm = format!("downloads/{}", &mod_nm);
    std::fs::create_dir_all(&parent_dir_nm).expect("Cannot Create parent Directory!");
    let set_dir_nm = format!("{}/{:03}", &parent_dir_nm, &set_nm);
    std::fs::create_dir_all(&set_dir_nm).expect("Cannot Create set Directory!");
    for file_number in 1..300 {
        let target = format!(
            "{}{:03}/{}{}{}{}{:03}.{}",
            url1, &set_nm, m_prepend, m_append, set_nm, joiner, file_number, config.file_suffix
        );
        downloader::download(&target, &client, &set_dir_nm).await?;
    }

    Ok(())
}
