pub mod hash_functions;
use hash_functions::HashableFunction;

#[derive(Debug, Clone)]
struct SubstringProfile(String, usize);

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
pub struct KmerTable {
    genome: String,
    kmer_table: Vec<Vec<SubstringProfile>>,
    hash_value: usize,
    kmer_size: usize,
}

/// Returns the genome of the kmer_table
///
/// # Arguments
///
/// * NA
///
impl KmerTable {
    /// Returns the genome of the kmer_table
    ///
    /// # Arguments
    ///
    /// * NA
    ///
    pub fn new(
        genome: String,
        hashtable_size: usize,
        //hash_function: &impl HashableFunction,
        kmer_size: usize,
    ) -> KmerTable {
        let inner_vec: Vec<SubstringProfile> = Vec::new();
        let hashtable = vec![inner_vec; hashtable_size + 1];
        let init_hash_value: usize = 0;
        KmerTable {
            genome: genome,
            kmer_table: hashtable,
            //hash_function: hash_function,
            hash_value: init_hash_value,
            kmer_size: kmer_size,
        }
    }

    /// Returns the genome of the kmer_table
    ///
    ///
    pub fn get_genome(&self) -> String {
        return self.genome.clone();
    }

    /// Returns the kmer size of the kmer_table
    ///
    ///
    pub fn get_kmer_size(&self) -> usize {
        return self.kmer_size;
    }

    pub fn push_value_to_table(
        &mut self,
        kmer: String,
        index_pos: usize,
        hash_function: &Box<dyn HashableFunction>,
    ) {
        // get hash value for kmer.
        //println!("{}, {}", kmer, self.hash_value);
        self.hash_value = hash_function.hash(&self.hash_value, &kmer);
        // add to array at hash value.
        //let kmer2 = kmer.clone();
        self.kmer_table[self.hash_value].push(SubstringProfile(kmer, index_pos));
        //println!("{:#?}", self.kmer_table[self.hash_value]);
        //println!("{}, {}", kmer2, self.hash_value);
    }

    pub fn check_if_kmer_in_table(
        &self,
        kmer: String,
        index_pos: usize,
        &hash_value: &usize,
        hash_function: &Box<dyn HashableFunction>,
    ) -> usize {
        // get hash value for kmer.
        let hash_value = hash_function.hash(&hash_value, &kmer);
        // check if kmer is in the table.
        let vec = &self.kmer_table[hash_value];
        //println!("{}, {}", kmer, kmer_hash_value);
        if vec.len() >= 1 {
            for substring_profile in vec.iter() {
                //println!("{}, {}, {}", kmer_hash_value, kmer, &substring_profile.0);
                if kmer.eq(&substring_profile.0) {
                    println!("{}, {}, {}", kmer, substring_profile.1, index_pos);
                }
            }
        }

        hash_value
    }

    pub fn find_matching_genome_kmers(
        &self,
        genome: String,
        hash_function: Box<dyn HashableFunction>,
    ) {
        // initalize kmer string and counter.
        let mut genome_kmer: String = "".to_string();
        let mut k_counter: usize = 0;
        let mut hash_value: usize = 0;
        // add kmers to the hash table.
        for character in genome.chars() {
            if k_counter < self.kmer_size {
                genome_kmer.push(character);
                k_counter += 1;
                // once equal to the kmer size, start checking against other file.
                if k_counter != self.kmer_size {
                    continue;
                }
            } else {
                genome_kmer.remove(0);
                genome_kmer.push(character);
                k_counter += 1;
            }
            let temp_kmer: String = genome_kmer.clone();
            let index_pos: usize = k_counter - self.kmer_size;
            hash_value =
                self.check_if_kmer_in_table(temp_kmer, index_pos, &hash_value, &hash_function);
        }
    }

    /// Class method
    /// Updates a KmerTable with the genome kmers it has for the current genome
    /// within it's fields.
    ///
    /// # Arguments
    /// 1. KmerTable instance
    ///
    /// # Returns
    /// 1. KmerTable - updated table
    /// # Examples
    ///
    /// ```rust
    /// kmer_table_1 = kmer_table::KmerTable::add_genome_to_table(kmer_table_1);
    /// ```
    pub fn add_genome_to_table(
        mut kmer_table: KmerTable,
        hash_function: Box<dyn HashableFunction>,
    ) -> KmerTable {
        // get fields from KmerTable instance.
        let genome: String = kmer_table.get_genome();
        let kmer_size: usize = kmer_table.get_kmer_size();
        // initalize kmer string and counter.
        let mut genome_kmer: String = "".to_string();
        let mut k_counter: usize = 0;
        // add kmers to the hash table.
        for character in genome.chars() {
            if k_counter < kmer_size {
                genome_kmer.push(character);
                k_counter += 1;
                // once equal to the kmer size, start checking against other file.
                if k_counter != kmer_size {
                    continue;
                }
            } else {
                genome_kmer.remove(0);
                genome_kmer.push(character);
                k_counter += 1;
            }
            let temp_kmer: String = genome_kmer.clone();
            let index_pos: usize = k_counter - kmer_size;
            //println!("{}, {}", temp_kmer, index_pos);
            kmer_table.push_value_to_table(temp_kmer, index_pos, &hash_function);
        }

        kmer_table
    }
}
