use reqwest;

pub async fn http_get(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(body)
}

pub fn scrape(html: &str) -> Vec<&str> {
    return vec![]
}