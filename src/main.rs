mod scrapper;
use async_trait::async_trait;
use std::borrow::Cow;
use clap::App; 

struct HtmlProcessorToJson {}

#[async_trait]
impl scrapper::client::ProcessHtmlString for HtmlProcessorToJson {
    async fn process_html(&self, page_num:i32, html: &String) -> Result<(), Box<dyn std::error::Error>>{
        let records = scrapper::scrape::scrape_html(html);
        let j = serde_json::to_string_pretty(&records)?;
        let file_name = format!("page_{}.json", page_num);
        scrapper::file::write_file(&file_name, &j).await?;
        return Ok(())
    }
}


#[derive(Debug, Default)]
struct HtmlProcessorToDb<'a> {
    db_path: Cow<'a, str>,
}

#[async_trait]
impl scrapper::client::ProcessHtmlString for HtmlProcessorToDb<'_>  {
    async fn process_html(&self, _page_num:i32, html: &String) -> Result<(), Box<dyn std::error::Error>>{
        let records = scrapper::scrape::scrape_html(html);
        scrapper::db::store_businesses(&self.db_path, &records)?;
        return Ok(())
    }
}

impl HtmlProcessorToDb<'_> {
    pub fn init(&mut self, path:&str) -> Result<(), Box<dyn std::error::Error>>{
        scrapper::db::initialize(path)?;
        self.db_path = Cow::Owned(path.to_string());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let url_gen = |i:i32| {
    //     if i > 150 {
    //         None
    //     }else{
    //         let url = format!("https://www.yellowpages.co.za/search?what=optometrist&pg={}", i);
    //         Some(url)
    //     }
    // };
    // let processor_tojson = HtmlProcessorToJson {};
    // let mut processor_todb = HtmlProcessorToDb::default();
    // processor_todb.init("./data.db3")?;
    // let result = scrapper::client::page_loop(url_gen, Box::new(processor_todb)).await;
    // match result {
    //     Ok(_) => {},
    //     Err(er) => println!("{:?}", er)
    // }
    

    App::new("myapp")
        .version("1.0")
        .about("Does great things!")
        .author("Kevin K.")
        .get_matches(); 
    Ok(())
}

