use std::{fmt::format, num};

use colored::Colorize;
use scraper::{ElementRef, Html};

/// Struct for storing the scraped news
#[derive(Debug)]
struct NewsHeadline {
    headline: String,
    link: String,
    time: String,
    num_points: Option<u32>,
    num_comments: Option<String>,
    author: Option<String>,
}

impl std::fmt::Display for NewsHeadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.headline.bold(),))?;

        f.write_str(&format!(
            "\n\t{} {} {} | {} ",
            self.num_points
                .map(|num_points| format!("{} points", num_points))
                .map(|num_points| num_points.magenta())
                .unwrap_or_default(),
            self.author
                .clone()
                .map(|author| format!("by {}", author))
                .map(|author| author.bright_black())
                .unwrap_or_default(),
            self.time.dimmed(),
            self.num_comments
                .clone()
                .map(|num_comments| num_comments.to_string())
                .map(|num_comments| num_comments.green())
                .unwrap_or_default()
        ))?;

        f.write_str(&format!("\n\t{}", self.link.underline().blue()))?;
        Ok(())
    }
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

        // Get all the elements with class "athing"
        let athing_selector = scraper::Selector::parse(".athing").unwrap();
        let athing_elements = document.select(&athing_selector);

        for (idx, athing_element) in athing_elements.enumerate() {
            // on the element with class `athing` get the span with class `.titleline`
            let titleline_selector = scraper::Selector::parse(".titleline").unwrap();
            let titleline_element = athing_element.select(&titleline_selector).next().unwrap();

            // get the first anchor element from `titleline` element
            let anchor_selector = scraper::Selector::parse("a").unwrap();
            let anchor_element = titleline_element.select(&anchor_selector).next().unwrap();

            // get the text of the anchor element
            // let heading = anchor_element.text().collect::<Vec<_>>().join("");
            let headline = anchor_element.text().collect::<String>();
            // get the value from `href` of the anchor element
            let link = anchor_element.value().attr("href").unwrap().to_string();

            // println!("{idx}:");
            // println!("   {:?}", &headline);
            // println!("   {:?}", &link);

            /******************************************************************************************************************/

            // get the next sibling of the athing element
            let athing_next_sibling = athing_element.next_sibling().unwrap();
            // convert the node to an element
            let athing_next_sibling_element = ElementRef::wrap(athing_next_sibling).unwrap();

            // get the span with class "age"
            let age_selector = scraper::Selector::parse(".age").unwrap();
            // get the text of the element with class ".age"
            let time = athing_next_sibling_element
                .select(&age_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // println!("   {:?}", &time);

            /*************************************************************************************************************************/

            // now get the element with class name "score"
            let score_selector = scraper::Selector::parse(".score").unwrap();
            let num_points = athing_next_sibling_element
                .select(&score_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // now retrieve the text present inside the `score_element`
            // let score = score_element.text().collect::<String>();
            // println!("   {:?}", &score);

            // get the element with class name "hnuser"
            let hnuser_selector = scraper::Selector::parse(".hnuser").unwrap();
            let author = athing_next_sibling_element
                .select(&hnuser_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // retrieve the text present inside the `huser_element`
            // let hnuser = hnuser_element.text().collect::<String>();
            // println!("   {:?}", &hnuser);

            /***********************************************************************************************************/
            // get the last child of subline element to get the number of comments in a news
            let subline_selector = scraper::Selector::parse(".subline > a:last-child").unwrap();
            let num_comments = athing_next_sibling_element
                .select(&subline_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // println!("   {:?}", &num_comments);

            self.news.push(NewsHeadline {
                headline,
                link,
                time: time.unwrap(),
                num_points: num_points.map(|num_points| {
                    num_points
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                }),
                num_comments,
                author,
            });
        }
        "".to_string()
    }

    pub fn get_news(&self) -> String {
        let mut news = String::new();
        news.push_str(&format!(
            "\n{}\n",
            " Hacker News ".bold().on_bright_green().black()
        ));

        for (i, news_headline) in self.news.iter().enumerate() {
            news.push_str(&format!("\n{}. {}", i + 1, news_headline));
        }

        news
    }
}
