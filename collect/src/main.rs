// Implement the function bubble_sort which receives a vector Vec<i32>
// and return the same vector but sorted in increasing order using the bubble
// sort algorithm

fn main() {
	let ref mut v = vec![3, 2, 4, 5, 1, 7];
	bubble_sort(v);
	println!("{:?}", v);
}

fn bubble_sort(v: &mut Vec<i32>) {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ordering() {
		let ref mut v = vec![3, 2, 4, 5, 1, 7];
		let mut b = v.clone();

		b.sort();
		bubble_sort(v);
		assert_eq!(*v, b);
	}
}
