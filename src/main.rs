mod kmer_table;
use kmer_table::hash_functions::HashableFunction;
mod arg_parser;
use std::env;

fn main() {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let fasta_paths = arg_parser::FastaPaths::new(&args);
    let mut use_rolling: bool = false;
    // parse fasta files
    let genome_1 = arg_parser::parse_fasta(&fasta_paths.fasta_1);
    let genome_2 = arg_parser::parse_fasta(&fasta_paths.fasta_2);
    // parse args
    let k: usize = args[3].clone().parse::<usize>().unwrap();
    if args.len() > 4 {
        if args[4].eq("--use_rolling") {
            use_rolling = true;
        }
    }
    // ingredients.
    let hashtable_size: usize = 20000000;
    let salt: usize = 20;
    let pattern_size: usize = k;

    // create kmer table
    let hash_fn: Box<dyn HashableFunction> = match use_rolling {
        true => Box::new(kmer_table::hash_functions::RollingHash::new(
            salt,
            hashtable_size,
            pattern_size,
        )),
        false => Box::new(kmer_table::hash_functions::RegularHash::new(
            salt,
            hashtable_size,
            pattern_size,
        )),
    };

    // create a kmer table data structure.
    let mut kmer_table_1 = kmer_table::KmerTable::new(genome_1, hashtable_size, k);

    // add first genome to custom hash table.
    kmer_table_1 = kmer_table::KmerTable::add_genome_to_table(kmer_table_1, hash_fn);

    // create kmer table
    let hash_fn: Box<dyn HashableFunction> = match use_rolling {
        true => Box::new(kmer_table::hash_functions::RollingHash::new(
            salt,
            hashtable_size,
            pattern_size,
        )),
        false => Box::new(kmer_table::hash_functions::RegularHash::new(
            salt,
            hashtable_size,
            pattern_size,
        )),
    };

    // check the second genome.
    kmer_table_1.find_matching_genome_kmers(genome_2, hash_fn);
}
