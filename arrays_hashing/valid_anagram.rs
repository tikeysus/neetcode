use std::collections::HashMap;

fn main(){
	println!("{}", is_anagram("hello".to_string(), "olleh".to_string())); 
}

fn is_anagram(s: String, t: String) -> bool{
	let mut map1 = HashMap::new();
	let mut map2 = HashMap::new();

	for letter in s.chars(){
		map1.entry(letter).and_modify(|x| *x += 1).or_insert(1); 
	}

	for letter in t.chars(){
		map2.entry(letter).and_modify(|x| *x += 1).or_insert(1); 
	}

	return map1 == map2; 
}