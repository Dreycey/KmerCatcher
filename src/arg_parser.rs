use std::fs;

pub fn parse_fasta(fasta_path: &String) -> String {
    //  open file
    let file_contents =
        fs::read_to_string(fasta_path).expect("Should have been able to read the file");
    // remove new line character
    let file_contents = file_contents
        .replace("\n", "")
        .replace(|c: char| !c.is_ascii(), ""); // rid of non-ascii characters

    file_contents
}

pub struct FastaPaths {
    pub fasta_1: String,
    pub fasta_2: String,
}

impl FastaPaths {
    pub fn new(args: &[String]) -> FastaPaths {
        // error handling
        if args.len() < 4 {
            panic!(
                "Not enough arguments, 
                    Usage: fasta_paths <fasta_1> <fasta_2> <kmer size> <--use_rolling [OPTIONAL]>"
            );
        }
        let fasta_1 = args[1].clone();
        let fasta_2 = args[2].clone();

        FastaPaths { fasta_1, fasta_2 }
    }
}
