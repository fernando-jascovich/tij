use cursive::views::{TextView, LinearLayout, Button, DummyView};

fn btns() -> LinearLayout {
    LinearLayout::horizontal()
        .child(Button::new("Fetch!", super::view_fetcher))
        .child(DummyView)
        .child(Button::new("No, thanks. This sucks man.", |a| a.quit()))
}

pub fn layer() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Hello, this is mutable and modularized TIJ!"))
        .child(TextView::new("Hitting the button will fetch data from https://jsonplaceholder.typicode.com/posts"))
        .child(DummyView)
        .child(btns())
}

