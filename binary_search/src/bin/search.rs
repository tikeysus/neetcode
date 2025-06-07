// There is an integer array nums sorted in ascending order (with distinct values).
// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
// You must write an algorithm with O(log n) runtime complexity.

fn main(){
	let nums: Vec<i32> = vec![4,5,6,7,0,1,2]; 
	let target: i32 = 5;
	println!("{}", search(nums, target)); 
}

fn search(nums: Vec<i32>, target: i32) -> i32{
	return search_helper(nums, target, 0); 
}

fn search_helper(nums: Vec<i32>, target: i32, acc: i32) -> i32{
	if nums.len() == 1{ //base case
		if nums[0] == target{
			return acc; 
		}
		return -1; 
	}
	
	let middle = nums.len()/2;
	let middle_elem = nums[middle]; 
	
	if middle_elem == target{
		return acc + (middle as i32); 
	} 

	let last = nums.len() - 1;
	let last_elem = nums[last]; 
	let lower_sub = nums[0..middle].to_vec(); 
	let upper_sub = nums[middle..=last].to_vec(); 

	if target < middle_elem && last_elem <= middle_elem{
		return search_helper(upper_sub, target, acc + (middle as i32)); 
	}

	else if target < middle_elem && last_elem > middle_elem{
		return search_helper(lower_sub, target, acc); 
	}

	else if target > middle_elem && last_elem <= middle_elem{
		return search_helper(lower_sub, target, acc + (middle as i32)); 
	}

	else if target > middle_elem && last_elem > middle_elem{
		return search_helper(upper_sub, target, acc); 
	}

	else{
		return -2; //debugging 
	}
}