//Write a function that reverses a string. The input string is given as an array of characters s.
//You must do this by modifying the input array in-place with O(1) extra memory.

fn main(){
	let mut s = vec!['h', 'e', 'l', 'l', 'o']; 
	reverse_string(&mut s); 
	println!("{:?}", s); 
}

fn reverse_string(s: &mut Vec<char>){
	let mut left = 0; 
	let mut right = s.len() - 1; 

	while left < right{
		let temp = s[left]; 
		s[left] = s[right]; 
		s[right] = temp; 

		left += 1;
		right -= 1;
	}
}