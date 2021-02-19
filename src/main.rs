extern crate dotenv;
mod gpt;

fn main() {
    // ensure .env file is present
    dotenv::dotenv().ok();
    let key = dotenv::var("GPT_KEY").unwrap();

    println!("got key {key}", key=key);
    let client = gpt::GPTClient{api_key: key};
    let text = "Recently, I've been reading 21 Lessons for the 21st Century written by none other than Yuval Noah Harari, and have been enjoying the book. Its approach to weaving together insights from a vast number of disciplines to create something novel is extremely refreshing. I just read a section on learning, the knowledge illusion, and collaborative thinking and wanted to share some of my thoughts that have been bouncing around and marinating for a few days as well as some learnings that I've applied to my own life.".parse().unwrap();
    let res = client.summarize(text);

    println!("summarized text: {}", res);
}
