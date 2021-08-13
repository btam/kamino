use std::process::exit;

use structopt::StructOpt;
mod command_line;
mod argument_types;
mod node_selection;
use command_line::*;
mod auto_update;
mod status;
mod update;

fn main() {
    env_logger::init();
    
    let opt = CommandLine::from_args();
    println!("{:#?}", opt);
    
    if let Err(err) = CommandLine::from_args().run() {
        eprintln!("Terminating with error: {:?}", err);
        exit(1);
    }
    else {
        println!("no error in CommandLine::from_args().run()");
    }
}