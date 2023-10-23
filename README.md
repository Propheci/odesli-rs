# odesli-rs

<div align="center">
  <a href="https://docs.rs/odesli-rs/">
    <img src="https://docs.rs/odesli-rs/badge.svg">
  </a>
  <a href="https://crates.io/crates/odesli-rs">
    <img src="https://img.shields.io/crates/v/odesli-rs.svg">
  </a>
</div>

[UNOFFICIAL] Async Rust library to communicate with Odesli API

- Supports getting by URLs and IDs

## Example

* In `Cargo.toml`

```toml
[package]
name = "odesli-test"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
odesli-rs = { git = "https://github.com/Propheci/odesli-rs" }
tokio = { version = "1.33.0", features = ["full"] }
```

* In `src/main.rs`
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = odesli_rs::ClientBuilder::default().build();

    dbg!(client.get_by_url("https://music.youtube.com/watch?v=cnnOwLfAxn0&si=3MtMRBN3Zy4FFNxU"));
    dbg!(
        client
            .get_by_id(
                "7CNUefGBVLn4cLoYv3ej8x",
                &odesli_rs::SupportedPlatform::Spotify,
                &odesli_rs::EntityType::Song
            )
            .await
    );

    Ok(())
}
```
