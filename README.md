# learn-rust

Setup
```
rustup update nightly
rustup default nightly
rustup component add rust-src
rustup component add rustfmt-preview
rustup component add clippy
```

Development
```
cargo fmt
cargo clippy
cargo build
cargo test
cargo run
```

### New projects
First create a new repository on [github](https://github.com/new)
```
git clone git@github.com:jplikesbikes/learn-rust.git newrepos
cd newrepos
git remote rename origin upstream
git remote add origin git@github.com:jplikesbikes/newrepos
git push -u origin master
```
Update the Cargo.toml name.
Update the readme.
Your good to go!
