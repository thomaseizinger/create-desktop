#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

/// An interactive utility program to create .desktop files from any executable.
#[derive(StructOpt, Debug)]
#[structopt(name = "create-desktop")]
struct Opt {
    /// The executable to create the .desktop file for.
    #[structopt(name = "executable", parse(from_os_str))]
    executable: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}