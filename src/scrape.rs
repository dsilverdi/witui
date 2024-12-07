use reqwest;
use scraper::{Html, Selector};

#[derive(Debug)]
pub enum ContentType {
    Links,
    Article
}

#[derive(Debug, PartialEq)]
pub enum TextType {
    Heading,
    SubHeading, 
    Paragraph
}

pub enum ScrapeResult {
    LinksResult(Vec<LinkElement>),
    Basic(Vec<ContentElement>)
}

#[derive(Debug)]
pub struct LinkElement {
    pub description: String,
    pub href: String,
}

#[derive(Debug)]
pub struct ContentElement {
    pub text: String,
    pub text_type: TextType
}

impl LinkElement {
    pub fn stringify(&self) -> String {
        format!("{} ({})", self.description, self.href)
    }
}

pub async fn http_get(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(body)
}

pub fn scrape(html: &str) -> Option<ScrapeResult> {
    let document = Html::parse_fragment(html);
    let body_content_selector = Selector::parse("#mw-content-text").unwrap();
    let search_result = Selector::parse("div.searchresults").unwrap();
    
    let mut content_type: ContentType = ContentType::Article;  
    if let Some(body_content_element) = document.select(&body_content_selector).next() {
        let result_element = body_content_element.select(&search_result).next();
        if let Some(p) = result_element {
            if p.inner_html() != "" {
                content_type = ContentType::Links;
            }
        }
    }

    match content_type {
        ContentType::Links => scrape_links(document),
        ContentType::Article => scrape_article(document)
    }
}

fn scrape_links(document: Html) -> Option<ScrapeResult> {
    let body_content_selector = Selector::parse("ul.mw-search-results").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let result_heading = Selector::parse("div.mw-search-result-heading").unwrap();

    let mut links: Vec<LinkElement> = vec![];
    if let Some(content_element) = document.select(&body_content_selector).next() {
        for li in content_element.select(&result_heading) {
            let text = li.text().collect();
            let href_val = li.select(&a_selector).next()
                .and_then(|a| a.value().attr("href"))
                .map(String::from)?;

            if starts_with_case_insensitive(&href_val, "/wiki/") {
                links.push(LinkElement {
                    description: text,
                    href: href_val,
                });
            }
        }
    }

    return Some(ScrapeResult::LinksResult(links))
}

fn scrape_article(document: Html) -> Option<ScrapeResult> {
    tracing::info!("scrape article {:?}", document);
    let first_heading = Selector::parse("#firstHeading").unwrap();
    let body_content_selector = Selector::parse("div.mw-content-ltr.mw-parser-output").unwrap();
    let content_selector = Selector::parse("p, .mw-heading2").unwrap();
    // let subheading_selector = Selector::parse(".mw-heading2").unwrap();

    let mut result: Vec<ContentElement> = vec![];
    let first_heading_el = document.select(&first_heading).next().unwrap().text().collect::<Vec<_>>().join("");
    result.push(ContentElement{
        text: first_heading_el,
        text_type: TextType::Heading,
    });

    if let Some(content) = document.select(&body_content_selector).next() {
        for element in content.select(&content_selector) {
            let tag_name = element.value().name();
            let text = element.text().collect::<Vec<_>>().join("");
            // result.push(text);

             // Skip empty paragraphs
            if text.trim().is_empty() {
                continue;
            }
        
            if tag_name == "p" {
                result.push(ContentElement{
                    text: text,
                    text_type: TextType::Paragraph,
                });
            } else {
                // For headings, find the actual h2 text
                if let Some(h2) = element.select(&Selector::parse("h2").unwrap()).next() {
                    result.push(ContentElement{
                        text:h2.text().collect::<String>(),
                        text_type:TextType::SubHeading,
                    });
                }
            }
        }
    }
    Some(ScrapeResult::Basic(result))
}

// fn contains_substr(s: &str, substr: &str) -> bool {
//     (0..substr.len())
//         .flat_map(|i| substr[i..].chars())
//         .any(|c| s.contains(c))
// }

fn starts_with_case_insensitive(main_string: &str, prefix: &str) -> bool {
    main_string.to_lowercase().starts_with(&prefix.to_lowercase())
}

// fn get_text_excluding_nested_li(element: &ElementRef) -> String {
//     let nested_li_selector = Selector::parse("li").unwrap();
//     element.text().filter(|&text| {
//         !element.select(&nested_li_selector).any(|nested_li| nested_li.text().any(|t| t == text))
//     }).collect::<Vec<_>>().join(" ").trim().to_string()
// }
