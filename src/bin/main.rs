use scdl::executor::run;

#[tokio::main]
async fn main() {
    run().await.expect("ERROR!");
}
