

pub fn insertion_sort(arr: &mut [isize]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn partition(arr: &mut [isize], pivot: isize) -> (usize, usize) {
    let mut high = arr.len();
    let mut low = 0;
    let mut mid = 0;

    while mid < high {
        match arr[mid] {
            x if x < pivot => {
                arr.swap(mid, low);
                low += 1;
                mid += 1;
                },
            x if x == pivot => mid += 1,
            _ => {
                arr.swap(mid, high - 1);
                high -= 1;
                }
            }
        }
    return (low, high);
}

#[test]
fn test_partiton() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (low, high) = partition(&mut arr, 5);
    assert_eq!(low, 4);
    assert_eq!(high, 5);
    assert_eq!(arr, [1, 2, 3, 4, 5, 7, 8, 9, 10, 6]);
}

fn median(a: isize, b: isize, c: isize) -> isize {
    if a < b {
        if b < c {
            b
        } else if a < c {
            c
        } else {
            a
        }
    } else {
        if a < c {
            a
        } else if b < c {
            c
        } else {
            b
        }
    }
}

#[test]
fn test_median() {
    assert_eq!(median(1, 2, 3), 2);
    assert_eq!(median(1, 3, 2), 2);
    assert_eq!(median(2, 1, 3), 2);
    assert_eq!(median(2, 3, 1), 2);
    assert_eq!(median(3, 1, 2), 2);
    assert_eq!(median(3, 2, 1), 2);
}

fn pivot(arr: &mut [isize]) -> isize {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let len = arr.len();
    median(arr[rng.gen_range(0..len)], arr[rng.gen_range(0..len)], arr[rng.gen_range(0..len)])
}

#[test]
fn test_pivot() {
    let mut unordered_vec: Vec<isize> = vec![1, 9, 4, 7, 2, 6, 3, 5, 4, 5, 8];
    let pivot = pivot(&mut unordered_vec);

    assert_eq!(unordered_vec.contains(&pivot), true);
}

pub fn quick_sort(arr: &mut [isize]) {
    if arr.len() <= 20 {
        insertion_sort(arr);
        return;
    }
    let pivot = pivot(arr);
    let (low, high) = partition(arr, pivot);
    quick_sort(&mut arr[0..low]);
    quick_sort(&mut arr[high..]);
}


