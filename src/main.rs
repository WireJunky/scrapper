mod scrapper;
use async_trait::async_trait;
use clap::{App, Arg};
use std::borrow::Cow;

struct HtmlProcessorToJson {}

#[async_trait]
impl scrapper::client::ProcessHtmlString for HtmlProcessorToJson {
    async fn process_html(
        &self,
        page_num: i32,
        html: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let records = scrapper::scrape::scrape_html(html);
        let j = serde_json::to_string_pretty(&records)?;
        let file_name = format!("page_{}.json", page_num);
        println!("writing to file: Data\\{}", file_name);
        scrapper::file::write_file(&file_name, &j).await?;
        return Ok(());
    }
}

#[derive(Debug, Default)]
struct HtmlProcessorToDb<'a> {
    db_path: Cow<'a, str>,
}

#[async_trait]
impl scrapper::client::ProcessHtmlString for HtmlProcessorToDb<'_> {
    async fn process_html(
        &self,
        _page_num: i32,
        html: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let records = scrapper::scrape::scrape_html(html);
        scrapper::db::store_businesses(&self.db_path, &records)?;
        return Ok(());
    }
}

impl HtmlProcessorToDb<'_> {
    pub fn init(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        scrapper::db::initialize(path)?;
        self.db_path = Cow::Owned(path.to_string());
        Ok(())
    }
}

fn buildQueryString(url:&str, what:&str, whre:&str, page:i32)->String{
    let mut query_string = String::from(url);
    query_string.push_str("/search?");
    query_string.push_str(format!("what={}", what).as_ref());
    if !whre.is_empty(){
        query_string.push_str(format!("&where={}", whre).as_ref());
    }
    query_string.push_str(format!("&pg={}",page).as_ref());
    return query_string;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = App::new("Scrapper")
        .version("1.0")
        .about("Scrapes www.yellowpages.co.za with custom search and output")
        .author("A. Coetzee - antoniecoetzee@gmail.com")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .possible_values(&["json", "db", "csv"])
                .default_value("db")
                .help("Sets the desired output type"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .default_value("./results")
                .help("Sets the output file - ./results.csv")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("what")
                .required(true)
                .short("w")
                .long("what")
                .value_name("WHAT")
                .help("Sets the 'what' query parameter - what=[parameter]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("where")
                .short("h")
                .long("where")
                .value_name("where")
                .default_value("")
                .help("Sets the 'where' query parameter - where=[parameter]")
                .takes_value(true),
        )        
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .value_name("START")
                .default_value("1")
                .help("Sets the start of the page range to scrape - &pg=[(start)..(end)]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .long("end")
                .value_name("END")
                .default_value("1")
                .help("Sets the end of the page range to scrape - &pg=[(start)..(end)]")
                .takes_value(true),
        )
        .get_matches();

    let mut html_proc: Option<Box<dyn scrapper::client::ProcessHtmlString>> = None;
    let output_type = matches.value_of("type").unwrap();
    match output_type {
        "json" => {
            html_proc = Some(Box::new(HtmlProcessorToJson {}));
        }
        "db" => {
            let output = matches.value_of("output").unwrap();
            let mut db_proc = HtmlProcessorToDb::default();
            db_proc.init(output)?;
            html_proc = Some(Box::new(db_proc));
        }
        "csv" => {}
        _ => {}
    }
    let start_page = matches.value_of("start").unwrap().parse::<i32>().unwrap();
    let end_page = matches.value_of("end").unwrap().parse::<i32>().unwrap();
    let url_gen = |current_page: i32| {
        if current_page > end_page {
            None
        } else {
            let url = buildQueryString("https://www.yellowpages.co.za", 
                        matches.value_of("what").unwrap(), 
                        matches.value_of("where").unwrap(),
                        current_page);
            println!("scraping [{}:{}]: {}", current_page, end_page, url);
            Some(url)
        }
    };
    let result = scrapper::client::page_loop(start_page, url_gen, html_proc.unwrap()).await;
    match result {
        Ok(_) => {}
        Err(er) => println!("{:?}", er),
    }

    Ok(())
}
