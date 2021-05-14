use cursive::{Cursive, CursiveRunnable};
use cursive::menu::MenuTree;
use cursive::views::Dialog;
use cursive::event::Key;

fn theme(app: &mut CursiveRunnable) {
    app.load_toml(include_str!("./theme.toml")).unwrap();
    app.set_autohide_menu(false);
}

fn keybindings(app: &mut CursiveRunnable) {
    app.add_global_callback('q', |a| a.quit());
    app.add_global_callback(Key::Esc, |s| s.select_menubar());
}

fn conf_edit() -> Dialog {
    Dialog::info("Edit this!")
}

fn conf_reset(app: &mut Cursive) {
    app.add_layer(Dialog::info("Not implemented yet, sorry."))
}

fn menu(app: &mut CursiveRunnable) {
    let conf = MenuTree::new()
        .leaf("Edit", |s| s.add_layer(conf_edit()))
        .leaf("Reset", conf_reset);
    let main = MenuTree::new()
        .subtree("Configuration", conf)
        .delimiter()
        .leaf("Quit", |s| s.quit());
    app.menubar().add_subtree("Main", main);
}

pub fn setup(app: &mut CursiveRunnable) {
    theme(app);
    keybindings(app);
    menu(app);
}
