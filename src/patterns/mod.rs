mod builder;
mod command;
mod new_type;
mod observer;
mod state_machine;

pub(crate) fn patterns() {
    observer::execute();
    builder::execute();
    command::execute();
    new_type::execute();
    state_machine::execute();
}
