extern crate dotenv;
mod gpt;

#[tokio::main]
async fn main() {
    // ensure .env file is present
    dotenv::dotenv().ok();
    let key = dotenv::var("GPT_KEY").unwrap();

    let client = gpt::GPTClient{api_key: key};
    let text = "Recently, I've been reading 21 Lessons for the 21st Century written by none other than Yuval Noah Harari, and have been enjoying the book.".parse().unwrap();
    let res = client.rephrase(text).await;
    println!("summarized text: {:#?}", res);

    let text = "Recently, I've been reading 21 Lessons for the 21st Century written by none other than Yuval Noah Harari, and have been enjoying the book.".parse().unwrap();
    let docs = vec![text];
    let res = client.search(docs, "person".parse().unwrap()).await;
    println!("matches {:#?}", res);
}
