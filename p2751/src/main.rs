use std::io::{self, Read, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = it.map(|s| s.parse().unwrap()).collect();
    quicksort(&mut nums, 0, n-1);

    for &e in nums.iter(){
        let _ = writeln!(writer, "{}", e);
    }
}

fn quicksort(arr: &mut [i32], mut left: usize, mut right: usize) {
    while left < right {
        median_of_three(arr, left, right);
        let pivot_idx = partition(arr, left, right);

        if pivot_idx - left < right - pivot_idx {
            if (pivot_idx > left) { quicksort(arr, left, pivot_idx-1); }
            left = pivot_idx + 1;
        } else {
            if (pivot_idx < right) { quicksort(arr, pivot_idx+1, right); }
            right = pivot_idx - 1;
        }
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;

    for j in left..right{
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, right);
    i
}

fn median_of_three(arr: &mut [i32], left: usize, right: usize){
    let mid = (left + right) / 2;

    if arr[left] > arr[mid] {
        arr.swap(left, mid);
    }
    if arr[left] > arr[right] {
        arr.swap(left, right);
    }
    if arr[mid] > arr[right] {
        arr.swap(mid, right);
    }

    arr.swap(mid, right);
}