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
- Has a CLI tool for the same: [Tools's README](./bin/README.md)

## Example

* In `Cargo.toml`

```toml
[package]
name = "odesli-test"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
odesli-rs = "5.1.0"
strum = "0.25.0"
tokio = { version = "1.33.0", features = ["full"] }
```

* In `src/main.rs`
```rust
use odesli_rs::{APIProvider, ClientBuilder, EntityType, Platform};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Supported Platforms:");
    for platform in Platform::iter() {
        println!(" - {:?}", platform)
    }
    println!("");

    println!("Supported API Providers:");
    for provider in APIProvider::iter() {
        println!(" - {:?}", provider)
    }
    println!("");

    let client = ClientBuilder::default()
        // .with_api_key(String::from("<INSERT_YOUR_API_KEY_HERE>")) // OPTIONAL
        // .with_api_version(String::from(odesli_rs::API_VERSION)) // Will be useful if any new API versions are released
        // .with_http_client(reqwest::Client::default()) // If you want to change your `reqwest::Client`'s settings
        .build();

    dbg!(
        client
            .get_by_url("https://music.youtube.com/watch?v=cnnOwLfAxn0")
            .await
    );

    let result = client
        .get_by_id(
            "7CNUefGBVLn4cLoYv3ej8x",
            &Platform::Spotify,
            &EntityType::Song,
        )
        .await?;

    dbg!(&result);
    dbg!(result.get_platform_url(&Platform::YouTube));
    dbg!(result.get_platform_entity(&Platform::YouTube));

    Ok(())
}
```
