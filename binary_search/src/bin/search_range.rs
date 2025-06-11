// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
// If target is not found in the array, return [-1, -1].
// You must write an algorithm with O(log n) runtime complexity.

fn main(){
	let nums = vec![5,7,7,8,8,10]; 
	let target = 8; 
	println!("{}", search_range(nums, target)); 
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32>{
	return search_range_helper(nums, target, 0); 
}

fn search_range_left(nums: Vec<i32>, target: i32, acc: i32) -> i32{
	if nums.len() == 0{
		return -1;
	}

	let middle: usize = nums.len() / 2; 
	if nums[middle] == target{
		
	}
}