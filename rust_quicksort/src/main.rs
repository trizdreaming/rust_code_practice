//use std::intrinsics;

fn main() {
	let mut test_target = [30, 50, 7, 40, 1000, 10000, 100, 88, 15, 44, 55, 22, 33, 77, 99, 11, 66, 1, 85, 100, 200, 300];
	let array_len = test_target.len();
	quick_sort(&mut test_target, array_len);
	println!("{:?}", test_target);
}

fn quick_sort(array: &mut [i32], array_len:usize) {
	// let array_len = array.len();
	// println!("array_len {:?}", array_len);

	if array_len <= 1 {
		return;
	}

	let pivot = partition(array, array_len);
	// println!("{:?}", array);
	// println!("{:?}", pivot);
	{
		let front_array = &mut array[0..pivot];
		// println!("{:?}", front_array);
		quick_sort(front_array, pivot);
		// println!("{:?}", front_array);
	}
	
	{
		let end_array = &mut array[pivot+1..array_len];
		// println!("{:?}", end_array);
		quick_sort(end_array, array_len-pivot-1);
		// println!("{:?}", end_array);	
	}
	

	// println!("{:?}{:?}{:?}",front_array, array[pivot], end_array );
}

fn partition(array: &mut [i32], array_len:usize) -> usize {
	let mut i:usize = 1;
	let mut j:usize = array_len-1;
	//println!("i idx {:?}", i);
	//println!("j idx {:?}", j);
	
	while i <= j {
		while array[i] < array[0] && i < array_len-1 {
			i += 1;
			//println!("i idx {:?}", i);
		}
		while array[j] >= array[0] && j > 0 {
			j -= 1;
			//println!("j idx {:?}", j);
		}
		if i < j {
			swap_contents(array, i, j);
		} else {
			swap_contents(array, 0, j);
		}
	}

	j
}

fn swap_contents(array: &mut [i32], target_idx1: usize, target_idx2: usize) {
	let temp:i32 = array[target_idx1];
	array[target_idx1] = array[target_idx2];
	array[target_idx2] = temp;
}

// fn print_type_of<T>(_: &T) {
// 	let type_for_print = 
// 		unsafe {
// 			(*std::intrinsics::type_name::<T>())
// 		};
// 	println!("{:?}", type_for_print);
// }
