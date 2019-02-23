use std::env;
use saucenao_api::SaucenaoRequestBuilder;

fn main() {
    let api_key = env::var("API_KEY").unwrap();
    let image = env::var("IMAGE_PATH").unwrap();

    let req = SaucenaoRequestBuilder::from_url(api_key, "https://i.imgur.com/1Io6JSO.png")
        .build();
    println!("{:?}", req.execute());
}