use odesli_rs::{LinksAPIResult, OdesliClient};

pub fn build_odesli_client(api_key: Option<String>) -> OdesliClient {
    let mut builder = odesli_rs::ClientBuilder::default();
    if let Some(api_key) = api_key {
        builder = builder.with_api_key(api_key);
    }
    builder.build()
}

pub fn pretty_print_api_result(result: &LinksAPIResult) {
    println!("SongLink Page: {}", result.page_url);

    println!("\n---");
    println!("Links From various platforms:");
    for (platform, link) in result.links_by_platform.iter() {
        println!("  {:?} ({}): {}", platform, link.entity_unique_id, link.url);
    }
    println!("---");

    println!("\n---");
    println!("Details from various platforms:");
    for entity in result.entities_by_unique_id.values() {
        let platform = &entity.api_provider;
        println!("\n  {:?}:", platform);

        let title = if let Some(title) = &entity.title { title } else { "<NA>" };
        println!("    Title: {}", title);

        let artists = if let Some(artists) = &entity.artist_name { artists } else { "<NA>" };
        println!("    Artist(s): {}", artists);

        let (thumbnail_url, thumbnail_width, thumbnail_height) =
            if let (Some(url), Some(width), Some(height)) =
                (&entity.thumbnail_url, entity.thumbnail_width, entity.thumbnail_height)
            {
                (url.as_str(), width, height)
            } else {
                ("<NA>", 0u64, 0u64)
            };
        println!("    Thumbnail({}x{}): {}", thumbnail_width, thumbnail_height, thumbnail_url);
    }
    println!("---");
}
