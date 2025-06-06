// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.

fn main(){
	let target = 5;
	let nums = vec![1,3,5,6]; 
	println!("{}", search_insert(nums, target)); 
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32{
	return search_insert_helper(nums, target, 0);
}

//cheeky helper function innit
fn search_insert_helper(nums: Vec<i32>, target: i32, acc: i32) -> i32{
	if nums.len() == 1 && target > nums[0]{
		return acc + 1; 
	}

	else if nums.len() == 1 && target <= nums[0]{
		return acc; 
	}

	if target < nums[nums.len() / 2]{
		let sub_array = nums[0..nums.len()/2].to_vec();
		return search_insert_helper(sub_array, target, acc); 
	}

	else{
		let sub_array = &nums[(nums.len() / 2)..nums.len()];
		return search_insert_helper(sub_array.to_vec(), target, acc + (nums.len()/2) as i32);
	}

}