use scraper::Html;
/// Struct for storing the scraped news
#[derive(Debug)]
struct NewsHeadline {
    headlin: String,
    link: String,
    time: String,
    num_points: Option<u32>,
    num_comments: Option<u32>,
    author: String,
}

/// struct for scrapping news
pub struct NewsScraper {
    news: Vec<NewsHeadline>,
}

/// implementation of NewsScraper
impl NewsScraper {
    pub fn new() -> Self {
        NewsScraper { news: Vec::new() }
    }

    pub fn scrape(&mut self, page: String) -> String {
        // parse the page info into DOM tree
        let document = Html::parse_document(&page);

        document.html().to_string()
    }
}
