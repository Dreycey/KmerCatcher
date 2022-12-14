pub trait HashableFunction {
    fn hash(&self, prev_hash: &usize, string2hash: &String) -> usize;
    //fn new(salt: usize, hash_size: usize, pattern_size: usize) -> Self;
}

#[derive(Debug, Clone)]
pub struct RegularHash {
    salt: usize,
    hash_size: usize,
}

impl RegularHash {
    /// constructor for a regular hash function
    ///
    ///
    pub fn new(salt: usize, hash_size: usize, _pattern_size: usize) -> RegularHash {
        // return instance.
        RegularHash {
            salt: salt,
            hash_size: hash_size,
        }
    }
}

impl HashableFunction for RegularHash {
    /// This hash is NOT rolling
    ///
    ///
    fn hash(&self, &_prev_hash: &usize, string2hash: &String) -> usize {
        let mut new_hash = 0; // NOT ROLLING
        for character in string2hash.chars() {
            let character_int: usize = character as usize;
            new_hash = (self.salt * new_hash + character_int) % self.hash_size;
        }

        new_hash
    }
}

#[derive(Debug, Clone)]
pub struct RollingHash {
    salt: usize,
    hash_size: usize,
    first_term_multiplier: usize,
}

impl RollingHash {
    /// constructor for a rolling hash function
    ///
    ///
    pub fn new(salt: usize, hash_size: usize, pattern_size: usize) -> RollingHash {
        // calculate the multiplier for the first term
        let mut first_term_multiplier: usize = 1;
        for _i in 0..pattern_size - 1 {
            first_term_multiplier = (salt * first_term_multiplier) % hash_size;
        }
        // return instance.
        RollingHash {
            salt: salt,
            hash_size: hash_size,
            first_term_multiplier: first_term_multiplier,
        }
    }
}
impl HashableFunction for RollingHash {
    fn hash(&self, &prev_hash: &usize, string2hash: &String) -> usize {
        // get first and last term of string.
        let mut string2hash2: String = string2hash.clone();
        let last_character_int: i32 = string2hash2.pop().expect("CHARACTER MISSING") as i32;
        let first_character_int: i32 =
            string2hash2.chars().nth(0).expect("CHARACTER MISSING") as i32;
        // calculate new hash.
        let mut new_hash: i32 = (((self.salt as i32)
            * ((prev_hash as i32)
                - ((first_character_int % (self.hash_size as i32))
                    * (self.first_term_multiplier as i32)))
            % (self.hash_size as i32))
            + (last_character_int % (self.hash_size as i32)))
            % (self.hash_size as i32);
        // if hash below 0, then move to correct value.
        if new_hash <= 0 {
            new_hash += (self.hash_size as i32);
        }

        new_hash as usize
    }
}
