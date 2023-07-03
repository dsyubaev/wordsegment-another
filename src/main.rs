use std::fs::File;
use std::io::{BufRead, BufReader};
use wordsegment_another;

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let unigrams = wordsegment_another::parse("unigrams.txt").unwrap();
    let bigrams = wordsegment_another::parse("bigrams.txt").unwrap();

    assert!(unigrams.contains_key("test"));
    assert!(bigrams.contains_key("in the"));

    let seg = wordsegment_another::Segmentator::new("unigrams.txt", "bigrams.txt", "words.txt");
    let a = seg.words[0..3].to_vec();
    println!("{:?}", a);
    //println!("{:?}", x?);
    let file = File::open("unigrams.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    let mut n = 0;
    for line in reader.lines() {
        println!("{}", line?);
        n += 1;
        if n > 10 {
            break;
        }
    }

    Ok(())
}
