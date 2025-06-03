//Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

use std::convert::TryInto;

fn main(){
	let target = 7; 
	let nums = vec![2,3,1,2,4,3];
	println!("{}", min_sub_array_len(target, nums));
	//min_sub_array_len(target, nums); 
}

fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32{ 
	let (mut right, mut left, mut sum) = (0, 0, 0); 
	let mut min = 0; 

	while right < (nums.len() - 1){
		if sum - nums[left] >= target{
			sum -= nums[left]; 
			left += 1; 
			
			if min == 0{
				min = right - left; 
			}
			
			else if right - left < min{
				min = right - left; 
			}
			
		}
		
		else{
			right += 1; 
			sum += nums[right]; 

			if min == 0{
				min = right - left; 
			}

			else if right - left < min{
				min = right - left; 
			}
		}
	}
	return min.try_into().unwrap();	
}