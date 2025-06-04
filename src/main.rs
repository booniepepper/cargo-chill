use cargo_chill::*;
use std::io;

fn main() -> io::Result<()> {
    let save_paths = SavePaths::from(get_or_create_save_dir().as_path());
    match aquire_lock(&save_paths) {
        Ok(()) => (),
        Err(LockError::AlreadyRunning(name, pid)) => {
            eprintln!("Already running as {:?} (PID: {})", name, pid);
            std::process::exit(1);
        }
    }

    let state = load_state(&save_paths)?;

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    save_state(&save_paths, &state)?;
    release_lock(&save_paths)?;

    ratatui::restore();
    app_result
}
