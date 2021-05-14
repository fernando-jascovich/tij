use cursive::{Cursive, CursiveRunnable};
mod home;
mod fetcher;

pub fn attach(app: &mut CursiveRunnable) {
    app.add_layer(home::layer())
}

fn view_fetcher(app: &mut Cursive) {
    app.pop_layer();
    app.add_layer(fetcher::layer());
}

