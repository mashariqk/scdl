use crate::configuration::get_configuration;
use crate::executor::ErrorKind::HttpRequest;
use error_chain::error_chain;
use std::f32::consts::E;
use std::fs::OpenOptions;
use std::io::Write;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download(url: &String, client: &reqwest::Client, fname: &String) -> Result<()> {
    let res = client.post(url).send().await?;
    match res.status().as_u16() {
        200 => {
            let mut dest = OpenOptions::new().write(true).create(true).open(fname)?;
            let buf = res.bytes().await?;
            dest.write(buf.as_ref())?;
            Ok(())
        }
        _ => panic!("Download error"),
    }
}
