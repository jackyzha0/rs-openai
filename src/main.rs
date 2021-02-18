extern crate dotenv;
mod gpt;

fn main() {
    // ensure .env file is present
    dotenv::dotenv().ok();
    let key = dotenv::var("GPT_KEY").unwrap();

    println!("got key {key}", key=key);
}
