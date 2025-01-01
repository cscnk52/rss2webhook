use std::error::Error;

use minreq::get;
use rss::{Channel, Item};

async fn fetch_rss(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = get(url).send()?;
    let channel = Channel::read_from(content.as_bytes())?;
    Ok(channel)
}

pub async fn get_first_item(url: &str) -> Result<Item, Box<dyn Error>> {
    let channel = fetch_rss(url).await?;
    let first = channel.items().first().unwrap();
    Ok(first.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch_rss() {
        let url = "https://sspai.com/feed";
        let result = get_first_item(url).await.unwrap();
        assert!(
            !result.title().is_none()
                || !result.link().is_none()
                || !result.description().is_none()
                || !result.author().is_none()
                || !result.categories().is_empty()
                || !result.comments().is_none()
                || !result.enclosure().is_none()
                || !result.guid().is_none()
                || !result.pub_date().is_none()
                || !result.source().is_none()
        );
    }
}
