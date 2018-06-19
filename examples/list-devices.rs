extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate pb_async;
extern crate tokio;

use futures::Future;

fn main() {
    env_logger::init();

    let token = dotenv::var("PUSHBULLET_TOKEN").expect("expected PUSHBULLET_TOKEN env var");

    tokio::run(futures::lazy(move || {
        let client = pb_async::Client::new(&token).unwrap();
        tokio::executor::spawn(client.list_devices().then(|res| {
            println!("Devices: {:#?}", res.unwrap());
            Ok(())
        }));

        Ok(())
    }))
}
