use fasta::read::FastaReader;
use std::path::Path;

// re insert a -> (String, String) in the function signature later
pub fn global_align(in_file: &Path) {
    //change fn signature to fasta_in: String
    // it is originally (seq1: &str, seq2: &str)
    // IDK if ill keep this here but this is to read the seqs from the input fasta
    
    //let infile = Path::new(&fasta_in);
    let mut seqs = Vec::new();
    for [_description, seq] in FastaReader::new(in_file) {
        seqs.push(seq);
    }

    let seq1 = &seqs[0];
    let seq2 = &seqs[1];
    // defining the scoring. in future this might be a struct, usable across other alignment
    // functions
    let gap = -1;
    let match_score = 1;
    let mismatch = -1;

    // need a matrix to store the scores
    let cols = seq2.len();
    let rows = seq1.len();

    let mut matrix = vec![vec![0; cols + 1]; rows + 1];
    
    // matrix of pointers to trace through the optimal alignment
    let mut trace_point = vec![vec![0; cols + 1]; rows + 1];

    // fill the first rows and columns of the matrices
    for i in 0..=rows {
        matrix[i][0] = i as i32 * gap;
        trace_point[i][0] = 3;
    }
    for j in 0..=cols {
        matrix[0][j] = j as i32 * gap;
        trace_point[0][j] = 4;
    }

    // variable to track if it was match or mismatch / gap in seq1 / gap in seq2
    let mut temp = vec![0, 0, 0]; 

    // populate the matrix 
    for i in 1..=rows {
        for j in 1..=cols {
            if seq1.chars().nth(i-1) == seq2.chars().nth(j-1) {
                temp[0] = matrix[i-1][j-1] + match_score;
            } else {
                temp[0] = matrix[i-1][j-1] + mismatch;
            }

            temp[1] = matrix[i-1][j] + gap;
            temp[2] = matrix[i][j-1] + gap;

            let temp_max = temp.iter().max().unwrap();
            matrix[i][j] = *temp_max;

            trace_point[i][j] += if temp[0] == *temp_max { 2 } else { 0 };
            trace_point[i][j] += if temp[1] == *temp_max { 3 } else { 0 };
            trace_point[i][j] += if temp[2] == *temp_max { 4 } else { 0 };
        }
    }

    // traceback
    let mut i = seq1.len();
    let mut j = seq2.len();
    let mut rx: Vec<char> = Vec::new();
    let mut ry: Vec<char> = Vec::new();

    while i > 0 || j > 0 {
        let trace_val = trace_point[i][j];

        if [2, 5, 6, 9].contains(&trace_val) {
            rx.push(seq1.chars().nth(i - 1).unwrap());
            ry.push(seq2.chars().nth(j - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if [3, 5, 7, 9].contains(&trace_val) {
            rx.push(seq1.chars().nth(i - 1).unwrap());
            ry.push('-');
            i -= 1;
        } else if [4, 6, 7, 9].contains(&trace_val) {
            rx.push('-');
            ry.push(seq2.chars().nth(j - 1).unwrap());
            j -= 1;
        }
    }

    let strx = rx.into_iter().rev().collect::<String>();
    let stry = ry.into_iter().rev().collect::<String>();

    println!("{}\n{}", strx, stry)
}
