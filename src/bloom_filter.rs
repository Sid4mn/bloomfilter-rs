use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A simple Bloom Filter implementation.
pub struct BloomFilter {
    // Bit vector representing the filter(each element is a bit).
    bits: Vec<bool>, // the bit array
    k: usize,        // number of hash functions
}

impl BloomFilter {
    // Create a new BF with a given size (no of bits) and no of hash functions.
    pub fn new(size: usize, hash_count: usize) -> Self {
        BloomFilter {
            bits: vec![false; size],
            k: hash_count,
        }
    }

    // Add an item into the BF.
    // This will set 'k' bits in the bit vector, one for each hash function.
    pub fn add<T: Hash>(&mut self, item: &T) {
        for i in 0..self.k {
            //For each hash func index from 0 to k-1, compute an index and set that bit.
            let index = self.hash_index(item, i);
            self.bits[index] = true;
        }
    }

    // Check if an item is possibly in the BF.
    // Returns TRUE if the item might be in the set (COULD BE FALSE POSITIVE)
    // or FALSE if the item is definitely not in the set.
    pub fn check<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.k {
            let index = self.hash_index(item, i);
            if !self.bits[index] {
                return false;
            }
        }
        true
    }

    // Compute a hash based index for a given item and hash function number 'i'.
    fn hash_index<T: Hash>(&self, item: &T, i: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(i);
        item.hash(&mut hasher);
        let hash_value = hasher.finish();

        // Map the hash val to a position in the bit vector
        (hash_value % (self.bits.len() as u64)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn basic_add_and_check() {
        let mut bf = BloomFilter::new(100, 3);
        bf.add(&"hello");
        bf.add(&"world");
        assert!(bf.check(&"hello"), "Bloom filter should contain 'hello'");
        assert!(bf.check(&"world"), "Bloom filter should contain 'world'");
        assert!(
            !bf.check(&"rust"),
            "Bloom filter should not contain 'rust' (not added)"
        );
    }

    #[test]
    fn test_false_positive() {
        let mut bf = BloomFilter::new(50, 3);
        bf.add(&"foo");
        bf.add(&"bar");
        let result = bf.check(&"baz");

        if result {
            println!("'baz' is possibly in the set (false positive).");
        } else {
            println!("'baz' is definitely not in the set (correct, likely outcome).");
        }

        assert!(bf.check(&"foo"));
        assert!(bf.check(&"bar"));
    }
}
