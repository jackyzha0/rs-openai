extern crate dotenv;
mod gpt;

fn main() {
    // ensure .env file is present
    dotenv::dotenv().ok();
    let key = dotenv::var("GPT_KEY").unwrap();

    println!("got key {key}", key=key);
    let client = gpt::GPTClient{api_key: key};
    let text = "If I put cheese in the fridge, will it melt?".parse().unwrap();
    let res = client.summarize(text);

    println!("summarize text: {}", res);
}
