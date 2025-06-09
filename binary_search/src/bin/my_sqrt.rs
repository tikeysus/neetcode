// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
// You must not use any built-in exponent function or operator.
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

fn main(){
	let x: i32 = 2147395599; 
	println!("{}", my_sqrt(x)); 
}

fn my_sqrt(x: i32) -> i32{
	let (mut low, mut high) = (0, 46340); //largest possible root given the test cases
	while low <= high{
		let mid: i32 = low + (high - low)/2; 
		if mid*mid == x{
			return mid; 
		}
		else if mid*mid > x{
			high = mid - 1; 	
		}
		else{
			low = mid + 1; 
		}
	}
	high
}