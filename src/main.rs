#![allow(unused)]

use std::mem::swap;

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

fn simple_pivot<T: PartialOrd + PartialEq>(slice: &[T]) -> usize {
    (slice.len() + 1) / 2
}

fn quick_sort<T: PartialOrd + PartialEq>(slice: &mut [T], pivot: fn(&[T]) -> usize) {
    if slice.len() == 2 {
        if slice[0] > slice[1] {slice.swap(0, 1);}
    } else if slice.len() > 2 {
        let n = slice.len();
        let p_i = pivot(slice);
        let nb_less = slice.iter()
            .filter(|x| **x <= slice[p_i])
            .count();
        slice.swap(p_i, nb_less-1);
        let mut pos_x: Vec<usize> = Vec::new();
        let mut pos_y: Vec<usize> = Vec::new();
        for i in 0..p_i {
            if slice[i] > slice[p_i] {
                pos_y.push(i);
            }
        }
        for i in (p_i+1)..n {
            if slice[i] <= slice[p_i] {
                pos_x.push(i);
            }
        }
        pos_x.reverse();
        pos_y.reverse();

        let mut it = pos_x.iter()
            .zip(pos_y.iter())
            .for_each(|(x, y)| slice.swap(*x, *y));

        quick_sort(&mut slice[0..p_i], pivot);
        quick_sort(&mut slice[(p_i+1)..n], pivot);
        
    }
}

fn merge_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(Debug)]
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
    fn test_quick_sort_1() {
        for query in generate_queries() {
            fn custom_quick_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) { quick_sort(slice, simple_pivot); }
            test_sort_variant!(custom_quick_sort, query.input, query.answer, query.qtype);
        }
    }
}
