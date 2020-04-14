use serde::{Deserialize, Serialize};
use scraper::{element_ref, Html, Selector};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BusinessRecord {
    pub name: String,
    pub address: String,
    pub address2: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub state: String,
    pub country: String,
    pub latitude: String,
    pub longitude: String,
    pub telephone: Vec<String>,
    pub email: Vec<String>,
}

impl BusinessRecord {
    fn from_div_element(&mut self, elem: element_ref::ElementRef) {
        if let Some(val) = BusinessRecord::single_from_selector(elem, "h5") {
            self.name = val;
        }

        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-address") {
            self.address = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-address2") {
            self.address2 = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-street") {
            self.street = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-city") {
            self.city = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-postalCode") {
            self.postal_code = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-state") {
            self.state = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-country") {
            self.country = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-latitude") {
            self.latitude = val;
        }
        if let Some(val) = BusinessRecord::single_from_selector(elem, ".yext-longitude") {
            self.longitude = val;
        }
        if let Some(val) = BusinessRecord::multiple_from_selector(elem, "#call_number a") {
            self.telephone = val;
        }
        if let Some(val) = BusinessRecord::multiple_from_selector(elem, ".row.pb-1 div:nth-of-type(3) .fullDetailId a") {
            self.email = val;
        }
    }

    fn single_from_selector(
        elem: element_ref::ElementRef,
        selector_string: &str,
    ) -> Option<String> {
        let selector = Selector::parse(selector_string).unwrap();
        match elem.select(&selector).next() {
            Some(v) => {
                let inner_html = v.inner_html();
                let trimmed = inner_html.trim();
                Some(String::from(trimmed))
            }
            None => None,
        }
    }

    fn multiple_from_selector(
        elem: element_ref::ElementRef,
        selector_string: &str,
    ) -> Option<Vec<String>> {
        let selector = Selector::parse(selector_string).unwrap();
        let mut result_vec = vec![];
        for child_elem in elem.select(&selector) {
            let inner_html = child_elem.inner_html();
            let trimmed = inner_html.trim();
            result_vec.push(String::from(trimmed));
        }
        Some(result_vec)
    }
}

pub fn scrape_html(html:&str)->Vec<BusinessRecord>{
    let mut result_vec = vec![];
    let doc = Html::parse_document(html);
    let selector = Selector::parse(".idBusinessDiv").unwrap();
    for element in doc.select(&selector) {
        let mut record = BusinessRecord::default();
        record.from_div_element(element);
        result_vec.push(record);
    }
    return result_vec;
}