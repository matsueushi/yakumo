// 閉路を検出する

#[derive(Debug, Clone)]
pub struct Graph {
    n: usize,
    data: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.data[from].push(to);
        self.data[to].push(from);
    }

    fn dfs(
        &self,
        used: &mut Vec<bool>,
        par: &mut Vec<Option<usize>>,
        from: Option<usize>,
        to: usize,
    ) -> Option<(usize, usize)> {
        par[to] = from;

        if used[to] {
            return Some((to, from.unwrap()));
        }
        used[to] = true;
        for v in &self.data[to] {
            if from == Some(*v) {
                continue;
            }
            let res = self.dfs(used, par, Some(to), *v);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    pub fn closed_path(&self, start: usize) -> Vec<usize> {
        let mut used = vec![false; self.n];
        let mut par = vec![None; self.n];
        let res = self.dfs(&mut used, &mut par, None, start);
        println!("{:?}", par);
        match res {
            Some((s, t)) => {
                let mut v = t;
                let mut nodes = vec![s];
                while v != s {
                    nodes.push(v);
                    v = par[v].unwrap();
                }
                println!("{:?}", nodes);
                nodes
            }
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed_path() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 4);
        let path = graph.closed_path(0);
        assert_eq!(path, vec![0, 2, 1]);
    }
}
