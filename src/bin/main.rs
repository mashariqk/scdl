use std::fs::File;
use std::io::{copy, Write};
use error_chain::error_chain;
use scdl::configuration::get_configuration;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_configuration().expect("Config failure!");
    let target = config.host_name;
    let client = reqwest::Client::new();
    let res = client
        .post(target)
        .send()
        .await?;
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
    let mut buf = res.bytes().await?;
    dest.write(buf.as_ref())?;
    Ok(())
}