//Given a string s, find the length of the longest substring without duplicate characters.

fn main(){
	let s: String = String::from("abcdefgabcd"); 
	println!("{}", length_of_longest_substring(s)); 
}

fn length_of_longest_substring(s: String) -> i32{
	let mut freq_counter: [u8; 256] = [0; 256]; 
	//Rust does not support indexing by number on type String due to encoding issues. 
	let bytes = s.as_bytes(); 
	let mut left = 0; 
	let mut right = 0; 
	let mut count = 0; 

	while right < bytes.len(){
		
		if freq_counter[bytes[right] as usize] == 1{
			freq_counter[bytes[left] as usize] = 0; 
			left += 1; 
		}
		
		else{
			freq_counter[bytes[right] as usize] = 1;
			right += 1; 
		}

		if right - left > count{
			count = right - left; 
		}
	}

	return count as i32; 
}