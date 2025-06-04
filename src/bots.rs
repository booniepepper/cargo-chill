use crate::state::Condition;
use std::str;

pub static CRATELIN: Bot = Bot {
    name: "Cratelin",
    portrait: unsafe { str::from_utf8_unchecked(include_bytes!("../art/bots/cratelin.txt")) },
    dialog: vec![],
    all_dialog: vec![],
};

#[derive(Debug)]
pub struct Bot {
    pub name: &'static str,
    pub portrait: &'static str,
    dialog: Vec<&'static str>,
    all_dialog: Vec<(Condition, Vec<&'static str>)>,
}
