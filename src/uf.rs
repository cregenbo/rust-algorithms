pub struct UF {
    id: Vec<usize>,
    count: usize,
}

impl UF {
    pub fn new(n: usize) -> Self {
        UF {
            id: (0..n).collect(),
            count: n,
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let idp = self.id[p];
        let idq = self.id[q];
        if idp != idq {
            for i in 0..self.id.len() {
                if self.id[i] == idp {
                    self.id[i] = idq;
                }
            }
            self.count -= 1;
        }
    }

    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

mod tests {
    use super::*;

    #[test]
    fn union_find_tests() {
        let mut uf = UF::new(10);
        assert_eq!(uf.count(), 10);
        uf.union(5, 3);
        assert!(uf.connected(5, 3));
        assert_eq!(uf.count(), 9);
        uf.union(2, 8);
        assert!(uf.connected(2, 8));
        assert_eq!(uf.count(), 8);
        uf.union(2, 3);
        assert!(uf.connected(5, 8));
        assert_eq!(uf.count(), 7);
    }
}
