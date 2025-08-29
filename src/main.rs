use rust_bloom_filter::BloomFilter;

fn main() {
    // Create a BloomFilter with 100 bits and 3 hash functions.
    let mut bloom = BloomFilter::new(100, 3);
    // Add some items to the bloom filter.
    bloom.add(&"apple");
    bloom.add(&"banana");

    // Check for the existence of some items.
    println!("Checking 'apple': {}", bloom.check(&"apple")); // true (was added)
    println!("Checking 'banana': {}", bloom.check(&"banana")); // true (was added)
    println!("Checking 'cherry': {}", bloom.check(&"cherry")); // false (not added)

    // Test for false positives with items that were never added
    let test_items = [
        "grape",
        "orange",
        "strawberry",
        "kiwi",
        "mango",
        "pineapple",
        "coconut",
    ];
    let mut false_positive_count = 0;

    println!("\nTesting for false positives:");
    for item in &test_items {
        let result = bloom.check(item);
        if result {
            println!("'{}': possibly in set (FALSE POSITIVE!)", item);
            false_positive_count += 1;
        } else {
            println!("'{}': definitely not in set (correct)", item);
        }
    }

    println!(
        "\nFalse positive rate: {}/{} = {:.2}%",
        false_positive_count,
        test_items.len(),
        (false_positive_count as f64 / test_items.len() as f64) * 100.0
    );

    // Test with a smaller bloom filter to increase false positive rate
    println!("\n--- Testing with smaller bloom filter (higher false positive rate) ---");
    let mut small_bloom = BloomFilter::new(5, 3); // Much smaller bit array
    small_bloom.add(&"apple");
    small_bloom.add(&"banana");

    false_positive_count = 0;
    for item in &test_items {
        let result = small_bloom.check(item);
        if result {
            println!("'{}': possibly in set (FALSE POSITIVE!)", item);
            false_positive_count += 1;
        } else {
            println!("'{}': definitely not in set (correct)", item);
        }
    }

    println!(
        "False positive rate with smaller filter: {}/{} = {:.2}%",
        false_positive_count,
        test_items.len(),
        (false_positive_count as f64 / test_items.len() as f64) * 100.0
    );
}
