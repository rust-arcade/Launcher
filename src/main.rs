use ui::launcher_ui;

mod core;
mod launcher_terminal;
mod ui;

fn main() {
    launcher_terminal::run();
    //ui::run();
}
