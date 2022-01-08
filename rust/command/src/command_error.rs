use failure::Fail;
use serde::{Deserialize, Serialize};

#[derive(Fail, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[fail(display = "Command execution error. Exit code: {} stderr: {}", _0, _2)]
// (exit_code, stdout, stderr)
pub struct CommandError(pub i32, pub String, pub String);
