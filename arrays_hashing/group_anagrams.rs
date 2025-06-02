use std::collections::HashMap;

fn main(){
	let strs: Vec<String> = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]; 
	println!("{:?}", group_anagrams(strs)); 
}

//this approach uses a frequency counter that maps to a vector featuring all the words under one palindromic category. 
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>{
	let mut my_dict: HashMap<[i32; 26], Vec<String>> = HashMap::new();
	
	for word in strs.iter(){
		let mut freq_counter: [i32; 26] = [0; 26]; 
		for letter in word.chars(){
			freq_counter[letter as usize - 'a' as usize] += 1;
		}
		my_dict.entry(freq_counter).and_modify(|value| value.push(word.to_string())).or_insert(vec![word.to_string()]); 
	}
	
	//cloned "owns" the item as it was previosuly a reference
	//collect then assembles it into a vector
	let res: Vec<Vec<String>> = my_dict.values().cloned().collect();
	return res; 
}