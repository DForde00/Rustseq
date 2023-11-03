mod global;
mod args;

use args:: {
    Command,
    Args
};

use structopt::StructOpt;

fn main() {

    let args = Args::from_args();

    let result = match args.cmdline {
       Command::Global(args) => global::global_align(&args.in_file)
        // the "ofile" arg to wrote output is not implemented yet
    };

    //let x = String::from("GATTACA");
    //let y = String::from("GCATGCU");
    //let res = global::global_align("./test.fasta");

    println!("{:?}", result);
}

