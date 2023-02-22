extern crate regex; // just for running in rust playground
use regex::Regex;

fn get_publish_date(title: String) -> String{
    let date_re = Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap();
    // let publish_date = date_re.captures(title.as_str()).unwrap().get(1).unwrap().as_str();
    // ----------------------------------------------------^^^^^^ here to match.
    let publish_date = match date_re.captures(title.as_str()) {
        Some(captured) => captured.get(1).unwrap().as_str(), // 这里unwrap()之后只有as_str()方法, 没有to_string()
        // None => format!("Unable to extract date from {}", title).as_str() // temporary value is freed at the end of this statement
        // None => "Unable to extract date from {title}" // str is equal to &'static str ?
        None => {
            let temp = format!("Unable to extract date from {}", title);
            temp.as_str()
        }
    };
    publish_date.to_string()
}

fn main() {
    let title = "【Rust Daily】2023-01-21"; // &str
    let publish_date = get_publish_date(title);
    println!("Title: {}\nPublish_data: {}\n", title, publish_date);
}