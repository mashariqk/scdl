use error_chain::error_chain;
use scdl::configuration::get_configuration;
use std::fs::OpenOptions;
use std::io::Write;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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
        let res = client.post(target).send().await?;
        match res.status().as_u16() {
            200 => {
                let mut dest = {
                    let fname = res
                        .url()
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .and_then(|name| if name.is_empty() { None } else { Some(name) })
                        .unwrap_or("tmp.jpg");
                    println!("File name is {}", &fname);
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(format!("{}/{}", &set_dir_nm, fname))?
                };
                let buf = res.bytes().await?;
                dest.write(buf.as_ref())?;
            }
            _ => break,
        }
    }

    Ok(())
}
