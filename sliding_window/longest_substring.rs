//Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k. if no such substring exists, return 0.

use std::collections::HashMap;

fn main(){
	let s = "aaabb".to_string();
	let k = 3; 
	println!("{}", longest_substring(s, k)); 
}

fn longest_substring(s: String, k: i32) -> i32{
	let (mut left, mut right, mut length) = (0, 0, 0); 
	let mut my_dict: HashMap<char, i32> = HashMap::new(); 
	
	while right < s.chars().count(){
		my_dict.entry(s.chars().nth(right)).and_modify(|x| *x += 1).or_insert(1); 

		
	}
}