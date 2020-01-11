use std::env::args;
use std::io::Error;

use futures::future::Future;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::MatrixClient;
use tokio_core::reactor::Core;

fn main() -> Result<(), std::io::Error> {
    let mut core = Core::new()?;

    let txns = MatrixClient::login_password(
        "crates.io".into(),
        &args()
            .nth(1)
            .expect("Missing password argument"),
        "https://matrix.org",
        &core.handle()
    ).map(|client| println!("{:?}", client.get_url()));

    core.run(txns).map_err(|merr| {
        println!("{}", merr);

        Error::last_os_error()
    }).expect("Failed to run txns");
    
    Ok(())
}
