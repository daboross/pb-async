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
                .upload_request("hello.txt", "text/plain", "Hello, world!\n".into())
                .and_then(move |file_data| {
                    client.push(
                        pb_async::PushTarget::SelfUser {},
                        pb_async::PushData::File {
                            body: "",
                            file_name: &file_data.file_name,
                            file_type: &file_data.file_type,
                            file_url: &file_data.file_url,
                        },
                    )
                })
                .or_else(|e| -> Result<_, _> {
                    panic!("error: {}", e);
                }),
        );
        Ok(())
    }));
}
