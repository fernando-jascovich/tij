mod app;
mod view;

pub fn run() {
    let mut app = cursive::default();
    app::setup(&mut app);
    view::attach(&mut app);
    app.run();
}
