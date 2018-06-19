pb-async
========
[![Linux Build Status][travis-image]][travis-builds]
[![Windows Build Status][appveyor-image]][appveyor-builds]

[![crates.io version badge][cratesio-badge]][pb-async-crate]

Asynchronous [Rust] PushBullet client.

Not official nor associated with PushBullet in any way.

---

`pb-async` provides a Futures interface to the [PushBullet v2 API].

Uses [`hyper`] and [`native-tls`] to make connections.

```rust
let token = std::env::var("PUSHBULLET_TOKEN")?;

let client = pb_async::Client::new(&token).unwrap();

tokio::run(
    client.push(
        pb_async::PushTarget::SelfUser {},
        pb_async::PushData::Note {
            title: "",
            body: "Hello, user!",
        },
    ).or_else(|error| {
        eprintln!("{}", error);
        Ok(())
    })
);
```

- [documentation][pb-async-docs]
- [crates.io page][pb-async-crate]

### API Completion

Implemented:
- authentication via user auth token
- list-devices: listing user devices
- create-push: creating a new push
- upload-request: uploading and pushing files
- get-user: retrieving user information

Not Implemented:
- retrieving tokens for other users via OAuth
- detailed device information in list-devices
- push management APIs
- device management APIs
- chat APIs
- subscription APIs

### Maintenance Status

`pb-async` is a hobby library. I intend to maintain the project and update for any future versions of `tokio`, `hyper` and the PushBullet API.

I will not, however, be implementing new features. Pull Requests will be appreciated, reviewed and accepted, but I have no other plans to further this library.

### Contributing

Contributions are welcome.

See [CONTRIBUTING](./CONTRIBUTING.md) for more information.

[Rust]: https://www.rust-lang.org/
[PushBullet v2 API]: https://docs.pushbullet.com
[`hyper`]: https://crates.io/crates/hyper
[`native-tls`]: https://crates.io/crates/native-tls
[travis-image]: https://travis-ci.org/daboross/pb-async.svg?branch=master
[travis-builds]: https://travis-ci.org/daboross/pb-async
[appveyor-image]: https://ci.appveyor.com/api/projects/status/ofdv9657k88jbpel/branch/master?svg=true
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/daboross/pb-async?branch=master&svg=true
[appveyor-builds]: https://ci.appveyor.com/project/daboross/pb-async
[cratesio-badge]: http://meritbadge.herokuapp.com/pb-async
[pb-async-docs]: https://docs.rs/pb-async/
[pb-async-crate]: https://crates.io/crates/pb-async
