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
        tokio::executor::spawn(
            client
                .push(
                    pb_async::PushTarget::SelfUser {},
                    pb_async::PushData::Note {
                        title: "User Greetings",
                        body: "Hello, user!",
                    },
                )
                .or_else(|e| -> Result<_, _> {
                    panic!("error: {}", e);
                }),
        );
        Ok(())
    }))
}
