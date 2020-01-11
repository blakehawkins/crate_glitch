use glitch_in_the_matrix::MatrixClient;
use tokio_core::reactor::Core;

fn main() -> Result<(), std::io::Error> {
    let core = Core::new()?;
    // let https = HttpsConnector::new(10)?;
    // let client = Client::builder().build::<_, hyper::Body>(https);

    let _fut = MatrixClient::login_password(
        "crates.io".into(),
        "password".into(),
        "matrix.org",
        &core.handle()
    );
    
    Ok(())
}
