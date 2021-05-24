use easy_rss::RssParser;
use std::collections::HashMap;

//use elefren::prelude::*;

const SECONDS_TO_MINUTES: u64 = 60;
const MINUTES_BETWEEN_CHECKS: u64 = 60;
const RSS_ADDRESS: &str = "https://omropfryslan.nl/rss";
const FILE_LOCATION: &str = "file.json";


//checks the rss feed, and only returns new elements if there are any
fn periodic_check(
    processed_items_list: &HashMap<std::string::String, std::string::String>,
) -> Vec<(std::string::String, std::string::String)> {
    let mut parser = RssParser::from_url(RSS_ADDRESS, "utf8").unwrap();
    let mut results = vec![];
    match parser.parse_vec() {
        Ok(item_list) => {
            for item in &item_list {
                if !processed_items_list.contains_key(&item.link) {
                    results.push((item.link.clone(), item.title.clone()));
                }
            }
        }
        _ => {}
    }
    results
}

fn add_status(status: &(std::string::String, std::string::String, Option<std::string::String>)) -> std::result::Result<(), ()> {
    

    Ok(())

}

fn mastodon_login() {

}

//gets a hashmap from file if there is one, returns a new one and creates the file if there is not.
fn get_processed_elements_hashmap() -> HashMap<std::string::String, std::string::String> {
    let processed_elements_hashmap: HashMap<std::string::String, std::string::String>;

    match std::path::Path::new(FILE_LOCATION).exists() {
        true => {
            processed_elements_hashmap =
                serde_json::from_str(std::fs::read_to_string(FILE_LOCATION).unwrap().as_str())
                    .unwrap();
        }
        false => {
            processed_elements_hashmap = HashMap::new();
            let serialized = serde_json::to_string(&processed_elements_hashmap).unwrap();
            std::fs::write(FILE_LOCATION, serialized).expect("Unable to write file");
        }
    }
    processed_elements_hashmap
}

fn main() {
    let mut processed_elements_hashmap = get_processed_elements_hashmap(); //: HashMap<std::string::String, std::string::String>;
    
    loop {
        println!("processing...");
        let new_elements = periodic_check(&processed_elements_hashmap);
        for new_item in &new_elements {
            match add_status(&(new_item.0.clone(),new_item.1.clone(),None)) {
                Ok(_) => {
                    processed_elements_hashmap.insert(
                        new_item.0.clone(),
                        new_item.1.clone(),
                    );
                    println!("updated hashmap, writing to disk");
                }
                Err(_) => {
                    println!("could not publish status");
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(
            1, //MINUTES_BETWEEN_CHECKS * SECONDS_TO_MINUTES,
        ));
    }

    let serialized = serde_json::to_string(&processed_elements_hashmap).unwrap();
    std::fs::write(FILE_LOCATION, serialized).expect("Unable to write file");
}