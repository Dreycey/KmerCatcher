mod kmer_table;
use kmer_table::hash_functions::HashableFunction;
mod arg_parser;
use std::env;

fn main() {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let fasta_paths = arg_parser::FastaPaths::new(&args);
    let mut use_rolling: bool = false;
    // parse fasta file
    let genome_1 = arg_parser::parse_fasta(&fasta_paths.fasta_1);
    let genome_2 = arg_parser::parse_fasta(&fasta_paths.fasta_2);
    // parse args
    let k: usize = args[3].clone().parse::<usize>().unwrap();
    if args[4].eq("--use_rolling") {
        use_rolling = true;
        println!("Using Rolling hash");
    }
    //
    let hashtable_size: usize = 20000000;
    let salt: usize = 20;
    let pattern_size: usize = k;

    // create kmer table
    //// create a hash table.
    let hash_fn = kmer_table::hash_functions::RollingHash::new(salt, hashtable_size, pattern_size);
    //// create a kmer table data structure.
    let mut kmer_table_1 = kmer_table::KmerTable::new(genome_1, hashtable_size, hash_fn, k);

    // testing the hash function.
    // let kmer_genome: String = kmer_table_1.get_genome();
    // println!("{}", kmer_genome);
    // kmer_table_1.push_value_to_table(String::from("ATGCGA"), 1);
    // kmer_table_1.push_value_to_table(String::from("ATGCTGA"), 1);
    kmer_table_1 = kmer_table::KmerTable::add_genome_to_table(kmer_table_1);
    // kmer_table_1.push_value_to_table(String::from("ATGCTGA"), 1);
    // kmer_table_1.check_if_kmer_in_table(String::from("ATGCCGA"), 2);
    kmer_table_1.find_matching_genome_kmers(genome_2);
}
