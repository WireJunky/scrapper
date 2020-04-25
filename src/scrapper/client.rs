use async_trait::async_trait;
use simple_error::SimpleError;

#[async_trait]
pub trait ProcessHtmlString {
    async fn process_html(
        &self,
        page_num: i32,
        html: &String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

async fn get(url_gen: impl Fn() -> String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url_gen()).await?;
    if resp.status().is_success() {
        let resp_text = resp.text().await?;
        Ok(resp_text)
    }else{
        if let Some(reason) = resp.status().canonical_reason() {
            Err(Box::new(SimpleError::new(reason)))
        }else{
            Err(Box::new(SimpleError::new("request error")))
        }     
    }
}

pub async fn page_loop(
    start_index:i32,
    url_gen: impl Fn(i32) -> Option<String>,
    html_processor: Box<dyn ProcessHtmlString>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut index = start_index;
    loop {
        if let Some(next_url) = url_gen(index) {
            let resp_text = get(|| next_url.clone()).await?;
            let proccessor = html_processor.as_ref();
            proccessor.process_html(index, &resp_text).await?;
            index = index + 1;
        } else {
            break;
        }
    }
    return Ok(());
}
