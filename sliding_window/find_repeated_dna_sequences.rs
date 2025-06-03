//The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
//For example, "ACGAATTCCG" is a DNA sequence.
//When studying DNA, it is useful to identify repeated sequences within the DNA.
//Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.

use std::collections::HashMap;

fn main(){
	let s = "AAAAAAAAAAA".to_string(); 
	println!("{:?}", find_repeated_dna_sequences(s)); 
}

fn find_repeated_dna_sequences(s: String) -> Vec<String>{
	let mut left = 0; 
	let mut right = 10; 
	let mut res: Vec<String> = Vec::new(); 
	let mut counter: HashMap<String, i32> = HashMap::new();

	if s.chars().count() < 10{
		return Vec::new(); 
	}

	while right <= s.chars().count(){
		let substring = &s[left..right]; 
		counter.entry(substring.to_string()).and_modify(|x| *x += 1).or_insert(1); 
		left += 1; 
		right += 1; 
	}

	for (key, value) in counter.iter(){
		if *value > 1{
			res.push(key.to_string()); 
		}
	}

	return res; 

}