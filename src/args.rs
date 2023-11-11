//! Argument definitions

//use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

// `structopt` for all arguments
#[derive(StructOpt)]
#[structopt(name = "rustseq",
            about = "A CLI package for sequence alignment",
            rename_all = "snake_case")]

pub struct Args {
    #[structopt(subcommand)]
    pub cmdline: Command
}

#[derive(StructOpt)]
pub enum Command {
    Global(GlobalOptions),
}

// available args for "GlobalOptions"
#[derive(StructOpt)]
#[structopt(name = "global alignment options",
            about = "reads file(s), aligns sequences using Needleman-wunsch algorithm",
            rename_all = "snake_case")]
pub struct GlobalOptions {
    #[structopt(help = "input file containing sequence(s)")]
    pub in_file: PathBuf, //,

    // Output file-writing not implemented yet
    //#[structopt(help = "output file to write")]
    //pub ofile: PathBuf,
    
    #[structopt(help = "tuple of match mismatch and gap scores in that order as positive integers, input 0,0,0 to use default scoring scheme")]
    pub m: i32,
    pub mm: i32,
    pub g: i32
}
