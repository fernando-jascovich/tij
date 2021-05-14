use serde_json::Value as SerdeValue;
use cursive::views::{TextView, LinearLayout, Button, DummyView};

fn external(url: &str) -> Result<SerdeValue, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(serde_json::from_str(&response)?)
}

fn posts() -> String {
    match external("https://jsonplaceholder.typicode.com/posts") {
        Ok(result) => result[0]["title"].to_string(),
        Err(e) => e.to_string()
    }
}

pub fn layer() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Here it is, the first title of the first post from https://jsonplaceholder.typicode.com/posts is:"))
        .child(TextView::new(posts()))
        .child(DummyView)
        .child(Button::new("Quit", |a| a.quit()))
}
