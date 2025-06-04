//! The game must go on and the numbers must go up.
//!
//! [State] will be designed and maintained as both:
//! 1. An in-memory representation of the game
//! 2. An on-disk snapshot of the game that can be resumed
//!
//! As such: always always always maintain backwards compatibility.
//!
//! Consider all future fields optional, and append-able. Avoid altering names
//! and spend a lot of time considering names.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};

pub type Condition = fn(State) -> bool;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub datetimes: DateTimes,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DateTimes {
    #[serde(default = "Local::now")]
    first_launch: DateTime<Local>,

    #[serde(skip_serializing, default = "Local::now")]
    current_launch: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(default)]
    launched_times: usize,

    /// 0 when the on-disk state is pristine and hasn't been tampered with.
    ///
    /// If tampering has been detected, the taint_level should be incremented
    /// for each time it has been fiddled with.
    #[serde(default)]
    taint_level: usize,
}

pub fn get_or_create_save_dir() -> PathBuf {
    let project = ProjectDirs::from("so.dang.cool", "CargoChill", "cargo-chill").unwrap();
    let data_dir = project.data_dir();

    if !data_dir.is_dir() {
        std::fs::create_dir_all(data_dir).expect(&format!(
            "Unable to create directory at: {}",
            data_dir.to_string_lossy()
        ));
    }

    data_dir.to_path_buf()
}

#[derive(Debug)]
pub struct SavePaths {
    state_json: PathBuf,
    digest: PathBuf,
    lock: PathBuf,
}

impl From<&Path> for SavePaths {
    fn from(save_dir: &Path) -> Self {
        let mut state_json = save_dir.to_path_buf();
        state_json.push("state.json");

        let mut lock = save_dir.to_path_buf();
        lock.push(".lock");

        let mut digest = save_dir.to_path_buf();
        digest.push(".digest");

        SavePaths {
            state_json,
            digest,
            lock,
        }
    }
}

pub enum LockError {
    AlreadyRunning(OsString, u32),
}

pub fn aquire_lock(paths: &SavePaths) -> Result<(), LockError> {
    // .lock (check pre-existing)
    if paths.lock.is_file() {
        if let Ok(pid) = std::fs::read_to_string(&paths.lock) {
            let pid = Pid::from(pid.trim().parse::<usize>().unwrap());

            let mut system = System::new_all();
            system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = system.process(pid) {
                return Err(LockError::AlreadyRunning(
                    process.name().to_owned(),
                    pid.as_u32(),
                ));
            } else {
                // Previous session exited without cleaning lock file. (Maybe crash?)
            };
        };
    }

    // .lock (create)
    let this_pid = std::process::id();
    std::fs::write(&paths.lock, this_pid.to_string()).unwrap();

    Ok(())
}

pub fn release_lock(paths: &SavePaths) -> std::io::Result<()> {
    std::fs::remove_file(&paths.lock)
}

/// Loads state from a given directory.
///
/// Three files will attempt to load:
///
/// 1. `state.json`: The state from a previous Chill session
/// 2. `.digest`: A SHA sum that helps detect people fiddling with the state
///
/// It will additionally create the following if they don't exist:
///
/// 1. `.lock`: Will be created with the current process ID
pub fn load_state(paths: &SavePaths) -> Result<State, std::io::Error> {
    // state.json
    let state_json = if paths.state_json.is_file() {
        std::fs::read_to_string(&paths.state_json).unwrap()
    } else {
        r#"{"datetimes":{},"metadata":{}}"#.into()
    };
    let mut state: State = serde_json::from_str(&state_json)
        .expect(&format!("Save file mangled {:?}", paths.state_json));

    state.metadata.launched_times += 1;

    // .digest
    match std::fs::read_to_string(&paths.digest) {
        Ok(digest) => {
            let expected = sha256::digest(&state_json);
            if digest != expected {
                state.metadata.taint_level += 1;
            }
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                if state.metadata.launched_times > 1 {
                    state.metadata.taint_level += 1;
                }
            }
            _ => return Err(e),
        },
    };

    Ok(state)
}

pub fn save_state(paths: &SavePaths, state: &State) -> Result<(), std::io::Error> {
    let state_json = serde_json::to_string(state).unwrap();
    std::fs::write(&paths.state_json, &state_json)?;

    let digest = sha256::digest(&state_json);
    std::fs::write(&paths.digest, &digest)?;

    Ok(())
}

#[test]
fn test_save_state() {
    let save_dir = get_or_create_save_dir();
    assert!(save_dir.is_dir());
}
