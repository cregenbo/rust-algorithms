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

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sort_tests() {
        let v1 = vec![3, 2, 5, 2, 1];
        assert_eq!(selection_sort(v1), [1, 2, 2, 3, 5]);
    }
}
