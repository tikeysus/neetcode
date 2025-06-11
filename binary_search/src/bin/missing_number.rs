// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

fn main(){
	let nums: Vec<i32> = vec![9,6,4,2,3,5,7,0,1]; 
	println!("{}", missing_number(nums)); 
}

fn missing_number(nums: Vec<i32>) -> i32{
	return missing_number_helper(nums, 0); 
}

fn missing_number_helper(nums: Vec<i32>, count: i32) -> i32{
	for num in &nums{
		if *num == count{
			return missing_number_helper(nums, count + 1);
		}
	}

	return count; 
}