enum Sorting {
    QuickSort,
    BubbleSort,
    MergeSort
}

fn quick_sort<T : PartialOrd + Copy>(array : &mut Vec<T>, low : usize, high : usize) {
    if low < high {
        let pivot : usize = partition(array, low, high);
        quick_sort(array, low, pivot-1);
        quick_sort(array, pivot+1, high);
    }
}

fn partition<T : PartialOrd + Copy>(array : &mut Vec<T>, low : usize, high : usize) -> usize {
    let pivot : T = array[high];
    let mut i = low;
    for j in low ..high {
        if array[j] < pivot {

            array.swap(i, j);
            i+=1;
        }
    }
    array.swap(i, high);
    i
}


fn merge_sort<T : PartialOrd + Copy>(array : &mut Vec<T>, l : usize, r: usize) {
    if l < r {
        let m : usize = (l + r) / 2;
        merge_sort(array, l, m);
        merge_sort(array,m+1, r);
        merge(array,l,m,r);
    }
} 

fn merge<T : PartialOrd + Copy>(array : &mut Vec<T>, left : usize, mid : usize, right : usize) {

    let mut temp :Vec<T> = Vec::with_capacity(right-left);
    let mut i : usize = left;
    let mut j : usize = mid+1;

    while i <= mid && j <= right {
        if array[i] <= array[j] {
            temp.push(array[i]);
            i+=1;
        }
        else {
            temp.push(array[j]);
            j+=1;
        }
    }

    while i <= mid {
        temp.push(array[i]); 
        i += 1;
    }

    while j <= right {
        temp.push(array[j]);
        j+=1;
    }

    for (k,val) in temp.iter().enumerate() {
        array[k+left] = *val;
    } 
}







fn bubble_sort<T : PartialOrd>(array : &mut Vec<T>) {
    for _ in 0..array.len() {
        for j in 0..array.len()-1 {
            if array[j] > array [j+1] {
                array.swap(j,  j+1);
            }
        }
    }
}

fn sort_with_chosen_algorithm<T : PartialOrd + Copy>(alogrithm : Sorting, array: &mut Vec<T>) {
    match alogrithm {
        Sorting::QuickSort => quick_sort(array, 0, array.len()-1),
        Sorting::BubbleSort => bubble_sort(array),
        Sorting::MergeSort => merge_sort(array, 0, array.len()-1)

    }
}

fn main () {
    let mut ar = vec!(4, 3, 5, 1, 2);
    sort_with_chosen_algorithm(Sorting::QuickSort, &mut ar);
    println!("{ar:?}");
    let mut ar = vec!('2', 'f', 'n', 'y', 'a', 'z', 'x');
    sort_with_chosen_algorithm(Sorting::BubbleSort, &mut ar);
    println!("{ar:?}");
    let mut ar = vec!(3.0, 4.4, -1.3, -1.2, -1.4, 0.0, 6.8);
    sort_with_chosen_algorithm(Sorting::BubbleSort, &mut ar);
    println!("{ar:?}");
}

#[test]
fn test_quick_sort() {
    let mut r = vec!(4, 3, 5, 1, 2);
    let len = r.len();
    quick_sort(&mut r, 0, len-1);
    let t = vec!(1, 2, 3, 4, 5);
    for i in 0..len {
        assert_eq!(r[i], t[i]);
    }
}

#[test]
fn test_merge_sort() {
    let mut r = vec!(4, 3, 5, 1, 2);
    let len = r.len();
    merge_sort(&mut r, 0, len-1);
    let t = vec!(1, 2, 3, 4, 5);
    for i in 0..len {
        assert_eq!(r[i], t[i]);
    }
}

#[test]
fn test_bubble_sort() {
    let mut r = vec!(4, 3, 5, 1, 2);
    let len = r.len();
    bubble_sort(&mut r);
    let t = vec!(1, 2, 3, 4, 5);
    for i in 0..len {
        assert_eq!(r[i], t[i]);
    }
}

