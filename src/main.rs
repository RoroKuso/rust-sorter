#![allow(unused)]

use std::{default, io::copy, mem::swap, fmt::{Debug, Error}};
use rand::Rng;

fn selection_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut ind = i;
        for j in (i + 1)..slice.len() {
            if slice[j] < slice[ind] {
                ind = j;
            }
        }
        slice.swap(i, ind);
    }
}

fn insertion_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn bubble_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut flag = false;
        for j in 1..(slice.len()-i) {
            if slice[j-1] > slice[j] {
                flag = true;
                slice.swap(j-1, j);
            }
        }
        if !flag { break; }
    }
}

fn partition_lomuto<T: PartialOrd + PartialEq>(slice: &mut [T]) -> Result<(usize, usize), Error> {
    let mut i: usize = 0;
    for j in 0..slice.len()-1 {
        if slice[j] <= *slice.last().ok_or(Error)? {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, slice.len()-1);
    Ok((i, i+1))
}

fn partition_hoare<T: PartialOrd + PartialEq + Clone>(slice: &mut [T]) -> Result<(usize, usize), Error> {
    let n: usize = slice.len();
    if n <= 2 { return Err(Error); }
    let mut i = 0;
    let mut j = n;
    let p: usize = rand::rng().random_range(0..n);
    let pivot = slice[p].clone();
    loop {
        loop {
            i += 1;
            if !(slice[i-1] < pivot) { break; }
        }
        loop {
            j -= 1;
            if !(slice[j] > pivot) { break; }
        }
        if (i-1) >= j { break; }
        slice.swap(i-1, j);
    }
    Ok((j+1, j+1))
}


fn quick_sort<T: PartialOrd + PartialEq>(slice: &mut [T], partition_f: fn(&mut [T]) -> Result<(usize, usize), Error>) -> Result<(), Error> {
    if slice.len() == 2 {
        if slice[0] > slice[1] {slice.swap(0, 1);}
    } else if slice.len() > 2 {
        let n = slice.len();
        let (i, j) = match partition_f(slice) {
            Ok((a, b)) => (a, b),
            Err(e) => return Err(e),
        };
        quick_sort(&mut slice[0..i], partition_f)?;
        quick_sort(&mut slice[j..n], partition_f)?;
    }
    Ok(())
}

fn merge_sort<T: PartialOrd + PartialEq + Clone + Debug>(slice: &mut [T]) {
    if slice.len() == 2 {
        if slice[0] > slice[1] { slice.swap(0, 1); }
    } else if slice.len() > 2 {
        let mid: usize = slice.len() / 2;
        merge_sort(&mut slice[0..mid]);
        merge_sort(&mut slice[mid..]);
        let mut i: usize = 0;
        let mut j: usize = 0;
        let left = slice[0..mid].to_vec().clone();
        let right = slice[mid..].to_vec().clone();
        while i < left.len() && j < right.len() {
            if left[i] > right[j] {
                slice[i+j] = right[j].clone();
                j += 1;
            } else {
                slice[i+j] = left[i].clone();
                i += 1;
            }
        }
        while i < left.len() {
            slice[i+j] = left[i].clone();
            i += 1;
        }
        while j < right.len() {
            slice[i+j] = right[j].clone();
            j += 1;
        }
    }
}


fn make_heap<T: PartialOrd + PartialEq>(slice: &mut [T], n: usize, i: usize) {
    let mut curr: usize = i;
    let left: usize = 2 * i + 1;
    let right: usize = 2 * i + 2;

    if left < n && slice[left] > slice[curr] {
        curr = left;
    }
    if right < n && slice[right] > slice[curr] {
        curr = right;
    }
    if curr != i {
        slice.swap(i, curr);
        make_heap(slice, n, curr);
    }
}

fn heap_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    let n: usize = slice.len();
    let start: usize = (n/2) - 1;
    for i in (0..=start).rev() {
        make_heap(slice, n, i);
    }

    for i in (1..n).rev() {
        slice.swap(0, i);
        make_heap(slice, i, 0);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum TestData {
        VecI64(Vec<i64>),
        VecF64(Vec<f64>),
        VecString(Vec<String>),
        VecStr(Vec<&'static str>),
    }

    enum QueryType {
        CheckEqual,
        CheckUnequal,
    }

    struct TestQuery {
        input: TestData,
        answer: TestData,
        qtype: QueryType,
    }

    fn generate_queries() -> Vec<TestQuery> {
        let data: Vec<TestQuery> = vec![
            TestQuery {
                input: TestData::VecI64(vec![1, 6, 3, 2]),
                answer: TestData::VecI64(vec![1, 2, 3, 6]),
                qtype: QueryType::CheckEqual,
            },
            TestQuery {
                input: TestData::VecI64(vec![3, 3, 1, 3]),
                answer: TestData::VecI64(vec![1, 3, 3, 3]),
                qtype: QueryType::CheckEqual,
            },
            TestQuery {
                input: TestData::VecI64(vec![7, 6, 7, 6, 1, 2, 3, 4, 3, 2, 1, 99, 98]),
                answer: TestData::VecI64(vec![1, 1, 2, 2, 3, 3, 4, 6, 6, 7, 7, 98, 99]),
                qtype: QueryType::CheckEqual,
            },
            TestQuery {
                input: TestData::VecStr(vec!["b", "a", "z", "c"]),
                answer: TestData::VecStr(vec!["a", "b", "c", "z"]),
                qtype: QueryType::CheckEqual,
            },
        ];
        return data;
    }

    macro_rules! assert_sort_variant {
        ($fun: ident, $input:expr, $expected:expr, $qtype:expr) => {
            let mut v = $input;
            $fun(&mut v);
            match $qtype {
                QueryType::CheckEqual => assert_eq!(v, $expected),
                QueryType::CheckUnequal => assert_ne!(v, $expected),
            }
        }
    }

    macro_rules! test_sort_variant {
        ($fun: ident, $input:expr, $expected:expr, $qtype:expr) => {
            match ($input, $expected) {
                (TestData::VecI64(a), TestData::VecI64(b)) => {
                    assert_sort_variant!($fun, a, b, $qtype);
                },
                (TestData::VecF64(a), TestData::VecF64(b)) => {
                    assert_sort_variant!($fun, a, b, $qtype);
                },
                (TestData::VecString(a), TestData::VecString(b)) => {
                    assert_sort_variant!($fun, a, b, $qtype);
                },
                (TestData::VecStr(a), TestData::VecStr(b)) => {
                    assert_sort_variant!($fun, a, b, $qtype);
                },
                (_, _) => continue,
            }
        };
    }

    #[test]
    fn test_selection_sort() {
        for query in generate_queries() {
           test_sort_variant!(selection_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_insertion_sort() {
        for query in generate_queries() {
           test_sort_variant!(insertion_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_bubble_sort() {
        for query in generate_queries() {
           test_sort_variant!(bubble_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_quick_sort_lomuto() {
        for query in generate_queries() {
            fn custom_quick_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) { 
                let _ = quick_sort(slice, partition_lomuto); 
            }
            test_sort_variant!(custom_quick_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_quick_sort_hoare() {
        for query in generate_queries() {
            fn custom_quick_sort<T: PartialOrd + PartialEq + Clone>(slice: &mut [T]) { 
                let _ = quick_sort(slice, partition_hoare); 
            }
            test_sort_variant!(custom_quick_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_merge_sort() {
        for query in generate_queries() {
           test_sort_variant!(merge_sort, query.input, query.answer, query.qtype);
        }
    }

    #[test]
    fn test_heap_sort() {
        for query in generate_queries() {
           test_sort_variant!(heap_sort, query.input, query.answer, query.qtype);
        }
    }
}
