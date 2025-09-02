use crate::command::Command;
use regex::Regex;

pub fn parse_shell_commands(content: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    let alias_re = Regex::new(r"^\s*alias\s+([a-zA-Z0-9_-]+)=").unwrap();
    let function_re = Regex::new(r"^\s*function\s+([a-zA-Z0-9_-]+)").unwrap();
    let function_paren_re = Regex::new(r"^([a-zA-Z0-9_-]+)\s*\(\)\s*\{?").unwrap();

    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(captures) = alias_re.captures(line) {
            if let Some(name) = captures.get(1) {
                commands.push(Command::new_alias(name.as_str().to_string()));
            }
        } else if let Some(captures) = function_re.captures(line) {
            if let Some(name) = captures.get(1) {
                commands.push(Command::new_function(name.as_str().to_string()));
            }
        } else if let Some(captures) = function_paren_re.captures(line) {
            if let Some(name) = captures.get(1) {
                let name_str = name.as_str();
                if !line.starts_with("alias ") {
                    commands.push(Command::new_function(name_str.to_string()));
                }
            }
        }
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::CommandType;

    #[test]
    fn test_parse_aliases() {
        let content = r#"
            alias ll='ls -la'
            alias gs='git status'
        "#;
        
        let commands = parse_shell_commands(content);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].name, "ll");
        assert_eq!(commands[0].command_type, CommandType::Alias);
        assert_eq!(commands[1].name, "gs");
        assert_eq!(commands[1].command_type, CommandType::Alias);
    }

    #[test]
    fn test_parse_functions() {
        let content = r#"
            function my_func() {
                echo "hello"
            }
            
            another_func() {
                echo "world"  
            }
        "#;
        
        let commands = parse_shell_commands(content);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].name, "my_func");
        assert_eq!(commands[0].command_type, CommandType::Function);
        assert_eq!(commands[1].name, "another_func");
        assert_eq!(commands[1].command_type, CommandType::Function);
    }

    #[test]
    fn test_parse_mixed_commands() {
        let content = r#"
            alias ll='ls -la'
            function test_func() {
                echo "test"
            }
            my_func() {
                echo "another"
            }
            alias gs='git status'
        "#;
        
        let commands = parse_shell_commands(content);
        assert_eq!(commands.len(), 4);
    }
}