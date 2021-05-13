use cursive::{Cursive, CursiveRunnable};
use cursive::views::{TextView, LinearLayout, Button, DummyView};
use serde_json::Value as SerdeValue;

fn keybindings(app: &mut CursiveRunnable) {
    app.add_global_callback('q', |a| a.quit());
}

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

fn text(app: &mut CursiveRunnable) {
    let btns = LinearLayout::horizontal()
        .child(Button::new("Fetch!", second))
        .child(DummyView)
        .child(Button::new("No, thanks. This sucks man.", |a| a.quit()));
    let layer = LinearLayout::vertical()
        .child(TextView::new("Hello, this is mutable and modularized TIJ!"))
        .child(TextView::new("Hitting the button will fetch data from https://jsonplaceholder.typicode.com/posts"))
        .child(DummyView)
        .child(btns);
    app.add_layer(layer)
}

fn second(app: &mut Cursive) {
    app.pop_layer();
    let layer = LinearLayout::vertical()
        .child(TextView::new("Here it is, the first title of the first post from https://jsonplaceholder.typicode.com/posts is:"))
        .child(TextView::new(posts()))
        .child(DummyView)
        .child(Button::new("Quit", |a| a.quit()));
    app.add_layer(layer)
}

pub fn run() {
    let mut app = cursive::default();
    keybindings(&mut app);
    text(&mut app);
    app.run();
}
