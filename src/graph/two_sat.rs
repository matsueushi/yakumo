use super::scc::SccGraph;

pub struct TwoSat {
    n: usize,
    graph: SccGraph,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            // 0, ..., n - 1: x_i = true
            // n, ..., 2n - 1: x_i = false
            graph: SccGraph::new(n << 1),
        }
    }

    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        // add clause (x_i = f) or (x_j = g)
        // -> (x_i != f => x_j = g) and (x_j != g => x_i = f)
        let i0 = if f { i + self.n } else { i };
        let j0 = if g { j } else { j + self.n };
        self.graph.add_edge(i0, j0);

        let j1 = if g { j + self.n } else { j };
        let i1 = if f { i } else { i + self.n };
        self.graph.add_edge(j1, i1);
    }

    pub fn satisfiable(&self) -> bool {
        let group = self.graph.scc();
        let mut cmp = vec![0; self.n << 1];
        for (i, nodes) in group.into_iter().enumerate() {
            for node in nodes {
                cmp[node] = i;
            }
        }
        for i in 0..self.n {
            if cmp[i] == cmp[i + self.n] {
                return false;
            }
        }
        true
    }

    pub fn answer(&self) -> Result<Vec<bool>, &str> {
        let group = self.graph.scc();
        let mut cmp = vec![0; self.n << 1];
        for (i, nodes) in group.into_iter().enumerate() {
            for node in nodes {
                cmp[node] = i;
            }
        }

        let mut res = vec![false; self.n];
        for i in 0..self.n {
            if cmp[i] == cmp[i + self.n] {
                return Err("No solution");
            } else {
                res[i] = cmp[i] > cmp[i + self.n];
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sat() {
        let mut two_sat = TwoSat::new(3);
        two_sat.add_clause(0, true, 1, false);
        two_sat.add_clause(1, true, 2, true);
        two_sat.add_clause(2, false, 0, false);
        assert!(two_sat.satisfiable());
        assert_eq!(two_sat.answer(), Ok(vec![true, true, false]));
    }
}
