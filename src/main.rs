mod global;
mod args;
mod scoring;

use args:: {
    Command,
    Args
};

use structopt::StructOpt;

fn main() {

    let args = Args::from_args();

    let result = match args.cmdline {
       Command::Global(args) => global::global_align(&args.in_file, (args.m, args.mm, args.g))
        // the "ofile" arg to write output is not implemented yet
    };

    println!("{:?}", result);
}

