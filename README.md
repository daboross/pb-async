pb-async
========
[![Linux Build Status][travis-image]][travis-builds]
[![Windows Build Status][appveyor-image]][appveyor-builds]

[![crates.io version badge][cratesio-badge]][pb-async-crate]

Asynchronous pushbullet client for [Rust].

Not official nor associated with PushBullet in any way.

---

`pb-async` provides a Futures interface to the [PushBullet v2 API].

Uses [`hyper`] and [`native-tls`] to make connections.

```rust
let token = std::env::var("PUSHBULLET_TOKEN")?;

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

API implemented:
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

`pb-async` is a hobby library maintained by @daboross. I intend to maintain the project into the future and upate to any future versions of `tokio`, `hyper` and the PushBullet API.

However, I will not be implementing new features. I'll review and include Pull Requests for new functionality, but will not work on any of it myself.

### Contributing

Contributions are welcome.

See [CONTRIBUTING](./CONTRIBUTING.md) for technical information on contributing.

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
