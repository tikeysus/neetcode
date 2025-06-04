//Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

fn main(){
	let target = 4; 
	let nums = vec![1,4,4];
	println!("{}", min_sub_array_len(target, nums));
}

fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32{ 
	let (mut left, mut right, mut sum, mut min) = (0, 0, 0, 0); 

	while right < nums.len(){
		sum += nums[right]; 
		right += 1; 

		while sum - nums[left] >= target{
			sum -= nums[left]; 
			left += 1; 
		}

		if min == 0 && sum >= target{
			min = right - left; 
		}

		else if right - left < min && sum >= target{
			min = right - left; 
		}
	}

	return min as i32; 
}