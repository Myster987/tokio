use super::Connection;
use crate::errors::CommandError;

const COMMAND_GET: &str = "GET";
const COMMAND_SET: &str = "SET";
const COMMAND_DELETE: &str = "DELETE";

pub enum Command {
    Get{ key: String},
    Set{ key: String, value: String},
    Delete{ key: String}
}

impl Command {
    pub async fn execute(&mut self, connection: &mut Connection) -> tokio::io::Result<()> {
        let payload: String;
        match self {
            Command::Get { key } => {
                payload = format!("GET \"{key}\"");                
            },
            Command::Set { key, value } => {
                payload = format!("SET \"{key}\" \"{value}\"");
            },
            Command::Delete { key } => {
                payload = format!("DELETE \"{key}\"");
            }
        }
        connection.write_to_stream(&payload).await?;

        Ok(())
    }

    pub async fn parse_to_command(input: String) -> Result<Command, CommandError> {
        let splited_command: Vec<&str> = input.split(" ").collect();

        match splited_command.as_slice() {
            [COMMAND_GET, key] => Ok(Command::Get { key: key.to_string() }),
            [COMMAND_SET, key, value] => Ok(Command::Set { key: key.to_string(), value: value.to_string() }),
            [COMMAND_DELETE, key] => Ok(Command::Delete { key: key.to_string() }),
            _ => Err(CommandError::CommandNotImplemented(input))
            
        }
    }
}