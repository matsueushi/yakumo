use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    node: usize,
    cost: usize,
}

#[derive(Debug, Clone)]
pub struct Graph {
    n: usize,
    data: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.data[from].push(Edge { node: to, cost });
    }

    pub fn dijkstra(&self, start: usize) -> Vec<Option<usize>> {
        let mut dist: Vec<_> = (0..self.n).map(|_| std::usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > dist[position] {
                continue;
            }

            for edge in &self.data[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        dist.into_iter()
            .map(|x| if x == std::usize::MAX { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 2, 10);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 1, 1);
        graph.add_edge(2, 3, 3);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 0, 7);
        graph.add_edge(3, 4, 2);

        let res = graph.dijkstra(0);
        assert_eq!(res, vec![Some(0), Some(1), Some(10), Some(3), Some(5)]);
        let res = graph.dijkstra(4);
        assert_eq!(res, vec![None, None, None, None, Some(0)]);
    }
}
