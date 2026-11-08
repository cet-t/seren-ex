mod common;

pub(crate) fn commands(flags: crate::CommonCommands) -> Vec<crate::Command> {
    let mut commands = Vec::new();
    if flags.has_flag(crate::CommonCommands::Ping) {
        commands.push(common::ping());
    }
    if flags.has_flag(crate::CommonCommands::Info) {
        commands.push(common::info());
    }
    if flags.has_flag(crate::CommonCommands::Greet) {
        commands.push(common::greet());
    }
    commands
}
