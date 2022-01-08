// use runner::Runner;
use std::{process::Command, str, thread, time::Duration};

use crate::CommandError;

#[macro_export]
macro_rules! command {
    // Making args, so we can just invoke it with command line only: command!(["echo", "hello"])
    ([$($x:expr),+ $(,)?]) => { command!([$($x),+], $crate::CommandArgs::default()) };

    // Generic case with args: command!(["echo", "hello"], CommandArgs{....})
    ([$($x:expr),+ $(,)?], $args:expr) => {
        {
            use $crate::AppendCmdImpl;
            let mut tmp_args = $args;
            let mut tmp_cmdline = Vec::<String>::new();
            // Building command line, by appending each item and collecting secret masks
            $( ($x).append_cmd(&mut tmp_cmdline, &mut tmp_args); )+
            // Invoking implementation function
            $crate::command_impl(tmp_cmdline, tmp_args)
        }
    };

    // Direct invocation of the implementation function with vector of strings
    ($vec_of_strings:expr, $args:expr) => { $crate::command::command_impl($vec_of_strings, $args) };
}

// Wrap any command argument X with MaskSecret(X) to prevent secret leaking into logs:
pub struct MaskSecret<'a, T: AppendCmdImpl + ?Sized + 'a>(pub &'a T);

pub struct CommandArgs {
    pub masq: Vec<String>,
    pub retries: u32,
    pub log_stdout: bool,
    pub log_stderr: bool,
    pub dry_run: bool,
}

impl Default for CommandArgs {
    fn default() -> CommandArgs {
        CommandArgs::default()
    }
}

impl CommandArgs {
    pub fn default() -> Self {
        CommandArgs {
            masq: Vec::new(),
            retries: 1,
            log_stdout: false,
            log_stderr: true,
            dry_run: false,
        }
    }
    pub fn new(
        masq: Vec<String>,
        retries: u32,
        log_stdout: bool,
        log_stderr: bool,
        dry_run: bool,
    ) -> Self {
        CommandArgs {
            masq,
            retries,
            log_stdout,
            log_stderr,
            dry_run,
        }
    }
}

pub fn command_impl(cmd: Vec<String>, args: CommandArgs) -> Result<String, CommandError> {
    // assert!(Runner::is_inside(), "Usage of command![] is only allowed inside Runner::run(), due to issues with signal handling");

    let mut masqed = format!("{:?}", cmd);
    for masq in args.masq {
        masqed = masqed.replace(&masq, "*****");
    }
    log::info!("==> {}", masqed);
    if args.dry_run {
        let output = format!("==> Command: dry-running command: {:?}", cmd);
        return Ok(output);
    }
    let mut retries = args.retries;
    loop {
        let output = Command::new(&cmd[0])
            .args(cmd[1..].as_ref())
            .output()
            .expect(&format!("Failed to execute: {:?}", cmd));

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if args.log_stdout && output.stdout.len() > 0 {
            log::info!("==> {}", stdout);
        }
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if args.log_stderr && output.stderr.len() > 0 {
            log::error!("==> {}", stderr);
        }
        if output.status.success() {
            return Ok(stdout);
        } else {
            retries = retries - 1;
            if retries > 0 {
                let base: i32 = 2;
                let delay: u64 = base.pow(args.retries - retries + 2) as u64;
                log::warn!(
                    "==> Command: waiting {} seconds before try {} of command: {:?}",
                    delay,
                    args.retries - retries + 1,
                    cmd
                );
                instrumented_sleep(Duration::from_secs(delay));
            } else {
                return Err(CommandError(
                    output.status.code().unwrap_or(-100),
                    stdout,
                    stderr,
                ));
            }
        }
    }
}

/// AppendCmdImpl trait allows to extend types supported by command! macro.
/// We implement it for bunch of common types below, including special MaskSecret(X) wrapper.
pub trait AppendCmdImpl {
    fn append_cmd(&self, dst: &mut Vec<String>, _args: &mut CommandArgs);
}

// Auto-implementing this trait for common types supporting .to_string():
// NOTE: can't use generic implementation, since it will implement it for undesired types
macro_rules! auto_impl {
    ($trait:ident, $type:ty) => {
        impl $trait for $type {
            fn append_cmd(&self, dst: &mut Vec<String>, _args: &mut CommandArgs) {
                dst.push(self.to_string());
            }
        }
    };
}

auto_impl!(AppendCmdImpl, char);
auto_impl!(AppendCmdImpl, str);
auto_impl!(AppendCmdImpl, &str);
auto_impl!(AppendCmdImpl, String);
auto_impl!(AppendCmdImpl, i32);
auto_impl!(AppendCmdImpl, u32);
auto_impl!(AppendCmdImpl, i64);
auto_impl!(AppendCmdImpl, u64);
auto_impl!(AppendCmdImpl, i8);
auto_impl!(AppendCmdImpl, u8);
auto_impl!(AppendCmdImpl, i16);
auto_impl!(AppendCmdImpl, u16);
auto_impl!(AppendCmdImpl, f32);
auto_impl!(AppendCmdImpl, f64);
auto_impl!(AppendCmdImpl, isize);
auto_impl!(AppendCmdImpl, usize);

// Auto-dereferencing automatically, as many times as needed, to further simplify referencing args and locals
impl<T: AppendCmdImpl> AppendCmdImpl for &T {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        (*self).append_cmd(dst, args);
    }
}

// This will flatten 2-tuple
impl<T: AppendCmdImpl, U: AppendCmdImpl> AppendCmdImpl for (T, U) {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        self.0.append_cmd(dst, args);
        self.1.append_cmd(dst, args);
    }
}

// This will skip over None option values
impl<T: AppendCmdImpl> AppendCmdImpl for Option<T> {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        if let Some(v) = self {
            v.append_cmd(dst, args)
        }
    }
}

// This will flatten arrays:
impl<T: AppendCmdImpl> AppendCmdImpl for [T] {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        for item in self.iter() {
            item.append_cmd(dst, args)
        }
    }
}

impl<T: AppendCmdImpl> AppendCmdImpl for Vec<T> {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        for item in self.iter() {
            item.append_cmd(dst, args)
        }
    }
}

// This allows to auto-track which secrets to mask
impl<'a, T: AppendCmdImpl + ?Sized + 'a> AppendCmdImpl for MaskSecret<'a, T> {
    fn append_cmd(&self, dst: &mut Vec<String>, args: &mut CommandArgs) {
        let pos = dst.len();
        self.0.append_cmd(dst, args);
        args.masq.extend(dst[pos..].to_owned());
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn command_works() {
        let out = command!(["echo", "hello world"]).expect("must succeed");
        // let foo = command!(["echo", "hello"]);
        assert_eq!(out, String::from("hello world\n"));
    }

    #[test]
    fn tuple_flattening() {
        let out = command!(["echo", ("hello", "world"),]).expect("mus succeed");
        assert_eq!(out, String::from("hello world\n"));
    }
}
/// For production code this is equivalent to thread::sleep()
/// In unit test mode when virtual time is activated and process execution is faked,
/// sleeping for retry doesn't achieve any differrent behavior and can't trigger new failures,
/// it will only slowing down a test, which is a pure waste of time.
/// So sleeps are skipped if virtual execution time mode is detected.
fn instrumented_sleep(duration: Duration) {
    let is_tokio_time_paused = tokio::time::Instant::now() == tokio::time::Instant::now();
    if !is_tokio_time_paused {
        thread::sleep(duration);
    }
}
