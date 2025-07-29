#[allow(unused)]
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

#[allow(unused)]
fn insertion_sort<T: PartialOrd + PartialEq>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    enum TestData {
        VecI64(Vec<i64>),
        VecF64(Vec<f64>),
        VecString(Vec<String>),
        VecStr(Vec<&'static str>),
    }

    #[allow(dead_code)]
    enum QueryType {
        CheckEqual,
        CheckUnequal,
    }

    #[allow(dead_code)]
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
}
