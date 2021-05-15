mod app;
mod view;
mod conf;

pub fn run() {
    let mut app = cursive::default();
    app::setup(&mut app);
    view::attach(&mut app);
    app.run();
}
