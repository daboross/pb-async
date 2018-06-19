### Contributing

#### Getting started (section in README)

Contributions are welcome!

I don't plan on introducing many new features into this library as it is primarily for my own use. However, if you need something added, I would be happy to review the code and accept PRs!

If you don't have the time to write a feature, feel free to also file an issue. If I don't add the feature, someone else could always see it and add it as well.

As a note, all contributions are expected to follow [the Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html).

#### Pull requests

Pull requests are _the_ way to change code using git. If you aren't familiar with them in general, GitHub has some [excellent documentation](https://help.github.com/articles/about-pull-requests/).

There aren't many hard guidelines in this repository on how specifically to format your request. Main points:

- Please include a descriptive title for your pull request, and elaborate on what's changed in the description.
- Feel free to open a PR before the feature is complete. Update PRs by committing directly to the PR branch.
  - This is good for reviewing PRs before merging
  - All commits will be squashed together on merge, so don't force push to the branch yourself.
- Please include at least a short description in each commit. Doesn't have to be much, but someone reading the history should be able to tell what's changed.
- If you have `rustfmt-preview` from a nightly toolchain installed, I recommend using it. If not, I'll reformat after merging your PR.

### Testing

As almost all functionality in this crate requires querying or modifying an active PushBullet
account, all inline tests are marked `no_run`.

In addition to running `cargo test`, I would recommend also running the example programs and
confirming behavior within a PushBullet client.

First, create `.env` containing

```sh
PUSHBULLET_TOKEN=your-token
```

Then run

```sh
cargo run --example get-user
cargo run --example list-devices
cargo run --example push-hello
cargo run --example upload-hello-world
```

### Mentoring

Feel free to email me at daboross @ daboross.net with any questions, or ping me with @daboross on GitHub.
