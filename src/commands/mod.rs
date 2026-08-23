mod common;

pub fn commands() -> Vec<crate::Command> {
    vec![common::ping(), common::info()]
}
