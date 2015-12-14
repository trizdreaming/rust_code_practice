

fn main() {
    let mut test_target = [0, 30, 50, 7, 40, 1, 60];
    let array_size = test_target.len() as usize;
    let mut heap_size = array_size-1;

    build_min_heap(&mut test_target, heap_size, array_size);
    println!("{:?}", test_target);

    let mut test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
    test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
    test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
    test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
    test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
    test = extract_min(&mut test_target, &mut heap_size);
    println!("{:?}", test);
}

// temporality
fn build_min_heap(array: &mut [i32], heap_size: usize, array_size: usize) {
	// let range = heap_size/2..0;
	// println!("{:?}", range);

	for x in (1..heap_size/2).rev() {
		println!("{:?}", x);
		min_heapify(array, x, array_size);
	}

	// for (x = heap_size/2; x > 0 ; --x) {
	// 	println!("{:?}", x);
	// }
}

// priority control
fn min_heapify(array: &mut [i32], idx: usize, array_size: usize) {
	let mut smallest: usize;

	if 2 * idx <= array_size && array[2 * idx] < array[idx] {
		smallest = 2 * idx;
	} else {
		smallest = idx;
	}

	if 2 * idx + 1 <= array_size && array[2 * idx + 1] < array[smallest] {
		smallest = 2 * idx + 1;
	}

	if smallest != idx {
		swap_contents(array, idx, smallest);
		min_heapify(array, smallest, array_size);
	}
}

fn swap_contents(array: &mut [i32], target_idx1: usize, target_idx2: usize) {
	let temp:i32 = array[target_idx1];
	array[target_idx1] = array[target_idx2];
	array[target_idx2] = temp;
}

// Pop
fn extract_min(array: &mut [i32], heap_size: &mut usize) -> i32{
	if *heap_size < 1 {
		println!("{:?}", "error!!");
		return -1;
	}

	let min = array[1];
	array[1] = array[*heap_size];
	*heap_size -= 1;

	min_heapify(array, 1, *heap_size);

	min
}

// push
fn insert(key: i32, array: &mut [i32], heap_size: &mut usize) {
	*heap_size += 1;
	let mut idx: usize = *heap_size;

	while idx > 1 && array[idx/2] > key {
		array[idx] = array[idx/2];
		idx = idx/2;
	}
	array[idx] = key;
}