extern crate rand;

pub fn insertion_sort(mut v: Vec<i32>) -> Vec<i32> {
    let mut j;
    for i in 1..v.len() {
        j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
    v
}

pub fn selection_sort(mut v: Vec<i32>) -> Vec<i32> {
    let mut min;
    for i in 0..v.len() {
        min = i;
        for j in i..v.len() {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
    v
}
#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sort_tests() {
        let mut rng = rand::thread_rng();
        let num_of_tests = 100;
        let size = 100;
        let mut v1: Vec<i32>;
        let mut v2: Vec<i32>;
        for i in 0..num_of_tests {
            v1 = (0..size).map(|_| rng.gen_range(-1000, 1000)).collect();
            v2 = v1.clone();
            v1.sort();
            assert_eq!(v1, selection_sort(v2.clone()));
            assert_eq!(v1, insertion_sort(v2.clone()));
        }
    }
}
