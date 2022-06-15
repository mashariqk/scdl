use error_chain::error_chain;
use scdl::configuration::get_configuration;
use std::fs::File;
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
    let set_nm = config.set_nm;
    let m_prepend = &config.mod_nm[..2];
    let m_append = config.mod_append;
    let joiner = config.joiner;
    let client = reqwest::Client::new();
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
                    File::create(fname)?
                };
                let buf = res.bytes().await?;
                dest.write(buf.as_ref())?;
            }
            _ => break,
        }
    }

    Ok(())
}
