use fasta::read::FastaReader;
use std::path::Path;

pub struct Fasta {
    Description: String,
    Seq: String,
}

pub fn read_fasta(fasta_in: String) {
    let infile = Path::new(&fasta_in);
    for [description, seq] in FastaReader::new(infile) {

    }
}
