mod news_scraper;
use news_scraper::NewsScraper;

#[tokio::main]
async fn main() {
    let url = "https://news.ycombinator.com/";
    let page = async { reqwest::get(url).await?.text().await }
        .await
        .unwrap();

    let mut news_scraper = NewsScraper::new();
    let news = news_scraper.scrape(page);
    println!("{}", news);
}
