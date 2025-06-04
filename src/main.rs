use cargo_chill::{
    state::{get_or_create_save_dir, load_state, save_state},
    *,
};
use std::io;

fn main() -> io::Result<()> {
    let save_dir = get_or_create_save_dir();
    let state = load_state(&save_dir).unwrap();

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    save_state(&save_dir, &state).unwrap();

    ratatui::restore();
    app_result
}
