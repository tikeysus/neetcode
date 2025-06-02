//Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
//Each letter in magazine can only be used once in ransomNote.

use std::collections::HashMap;

fn main(){
	let ransom_note: String = String::from("aa"); 
	let magazine: String = String::from("aab"); 
	println!("{}", can_construct_array(ransom_note, magazine)); 
}

//will try first with a HashMap approach
//each letter is mapped to an array of length two, one index indicating the frequency of that letter in magazine,
//the other - in ransomNote. 
//this is a heavy solution as HashMap are pretty expensive for memory. 
fn can_construct_hashmap(ransom_note: String, magazine: String) -> bool{
	let mut my_dict: HashMap<char, [i32; 2]> = HashMap::new(); 

	for letter in ransom_note.chars(){
		my_dict.entry(letter).and_modify(|key| key[0] += 1).or_insert([1,0]);
	}

	for letter in magazine.chars(){
		my_dict.entry(letter).and_modify(|key| key[1] += 1).or_insert([0, 1]);
	}

	let dict_values = my_dict.values(); 

	for values in dict_values{
		if values[0] > values[1]{
			return false; 
		}
	}

	return true; 
}

//simpler approach using arrays, code is self-explanatory
fn can_construct_array(ransom_note: String, magazine: String) -> bool{
	let mut notes_vals: [i32; 26] = [0; 26];
	let mut magazine_vals: [i32; 26] = [0; 26];

	for letter in ransom_note.chars(){
		notes_vals[letter as usize - 'a' as usize] += 1;
	}

	for letter in magazine.chars(){
		magazine_vals[letter as usize - 'a' as usize] += 1;
	}

	for i in 0..notes_vals.len(){
		if notes_vals[i] > magazine_vals[i]{
			return false; 
		}
	}

	return true; 
}