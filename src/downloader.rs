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

pub async fn download(url: &String, client: &reqwest::Client, set_dir_nm: &String) -> Result<()> {
    let res = client.post(url).send().await?;
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
        _ => Err("non 200 code".into()),
    }
}

fn extract_name_from_url(url: &String) -> &str {
    let index = url.rfind('/').expect("No / found in URL") + 1;
    &url[index..]
}
