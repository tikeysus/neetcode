// You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array fruits where fruits[i] is the type of fruit the ith tree produces.
// You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:
// You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
// Starting from any tree of your choice, you must pick exactly one fruit from every tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
// Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
// Given the integer array fruits, return the maximum number of fruits you can pick.

use std::collections::HashMap;
use std::convert::TryInto;

fn main(){
	let fruits = vec![1,2,3,2,2];
	println!("{}", total_fruit(fruits)); 
}

fn total_fruit(fruits: Vec<i32>) -> i32{
	let (mut left, mut right, mut max) = (0, 0, 0); 
	let mut hash: HashMap<i32, i32> = HashMap::new(); 

	while right < fruits.len(){
		hash.entry(fruits[right]).and_modify(|x| *x+=1).or_insert(1); 
		right += 1; 

		while hash.len() > 2 {
			if let Some(&count) = hash.get(&fruits[left]) {
				if count > 1 {
					if let Some(left_key) = hash.get_mut(&fruits[left]) {
						*left_key -= 1;
					}
				} 
				else if count == 1 {
					hash.remove(&fruits[left]);
				}
			}
			left += 1;
		}

		if right - left > max{
			max = right - left; 
		}
	}
	return max.try_into().unwrap(); 
}