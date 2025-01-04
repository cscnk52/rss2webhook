use std::error::Error;

use minreq::get;
use rss::{Channel, Item};

pub async fn fetch_rss(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = get(url).send()?;
    let channel = Channel::read_from(content.as_bytes())?;
    Ok(channel)
}

pub async fn get_first_item(channel: Channel) -> Result<Item, Box<dyn Error>> {
    let first = channel.items().first().unwrap();
    Ok(first.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch_rss() {
        let url = "https://sspai.com/feed";
        let channel = fetch_rss(url).await.unwrap();
        let result = get_first_item(channel).await.unwrap();
        assert!(
            result.title().is_some()
                || result.link().is_some()
                || result.description().is_some()
                || result.author().is_some()
                || !result.categories().is_empty()
                || result.comments().is_some()
                || result.enclosure().is_some()
                || result.guid().is_some()
                || result.pub_date().is_some()
                || result.source().is_some()
        );
    }
}
