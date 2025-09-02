#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Alias,
    Function,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
}

impl Command {
    pub fn new_alias(name: String) -> Self {
        Self {
            name,
            command_type: CommandType::Alias,
        }
    }

    pub fn new_function(name: String) -> Self {
        Self {
            name,
            command_type: CommandType::Function,
        }
    }
}