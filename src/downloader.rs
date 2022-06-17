use crate::configuration::get_configuration;
use error_chain::error_chain;
use std::fs::OpenOptions;
use std::io::Write;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download_set(set_nm: u16) -> Result<()> {
    let config = get_configuration().expect("Config failure!");
    let url1 = config.get_url_to_append_set();
    let mod_nm = config.mod_nm;
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
        download(&target, &client, &set_dir_nm).await?;
    }
    Ok(())
}

pub async fn download(url: &String, client: &reqwest::Client, set_dir_nm: &String) -> Result<()> {
    let mut counter = 0;
    let mut response = match client.post(url).send().await {
        Ok(r) => Ok(r),
        Err(e) => {
            counter += 1;
            println!(
                "Retry count is now {} and error is {} for url {}",
                counter, e, &url
            );
            Err(e)
        }
    };
    while counter < 3 && response.is_err() {
        response = match client.post(url).send().await {
            Ok(r) => Ok(r),
            Err(e) => {
                counter += 1;
                println!(
                    "Retry count is now {} and error is {} for url {}",
                    counter, e, &url
                );
                Err(e)
            }
        };
    }
    let res = response?;
    match res.status().as_u16() {
        200 => {
            let fname = extract_name_from_url(url);
            let mut dest = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("{}/{}", set_dir_nm, fname))?;
            let buf = res.bytes().await?;
            dest.write(buf.as_ref())?;
            Ok(())
        }
        _ => Err(format!(
            "HTTP code {} received for url: {}",
            res.status().as_u16(),
            url
        )
        .into()),
    }
}

fn extract_name_from_url(url: &String) -> &str {
    let index = url.rfind('/').expect("No / found in URL") + 1;
    &url[index..]
}
