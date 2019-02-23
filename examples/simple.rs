use std::env;
use saucenao_api::SaucenaoRequestBuilder;

fn main() {
    let api_key = env::var("API_KEY").unwrap();
    let image = env::var("IMAGE_PATH").unwrap();

    let req = SaucenaoRequestBuilder::from_file(api_key, image)
        .build();
    println!("{:?}", req.execute());
}