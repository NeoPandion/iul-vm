use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "iul-vm", about = "A pedagogical VM for learning Java")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}
