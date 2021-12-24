use ahash::AHashMap;
use std::{
    collections::{hash_map::Entry, BinaryHeap},
    hash::Hash,
    ops::Add,
};

pub struct AStarInfo<N, C> {
    pub path: Vec<(N, C)>,
    pub total_cost: C,
}

pub fn astar<N, C, FN, FH, FC>(
    start: N,
    mut next: FN,
    mut heuristic: FH,
    mut is_target: FC,
) -> Option<AStarInfo<N, C>>
where
    N: Clone + Hash + Eq,
    C: Ord + Copy + Add<Output = C> + Default,
    FN: FnMut(&N, &mut Vec<(N, C)>),
    FH: FnMut(&N) -> C,
    FC: FnMut(&N) -> bool,
{
    struct Pending<N, C: Ord + Copy + Add<Output = C> + Default> {
        cost: C,
        cost_and_heuristic: C,
        node: N,
        previous: Option<N>,
    }

    impl<N, C: Ord + Copy + Add<Output = C> + Default> PartialEq for Pending<N, C> {
        fn eq(&self, other: &Self) -> bool {
            self.cost_and_heuristic == other.cost_and_heuristic
        }
    }
    impl<N, C: Ord + Copy + Add<Output = C> + Default> Eq for Pending<N, C> {}
    impl<N, C: Ord + Copy + Add<Output = C> + Default> PartialOrd for Pending<N, C> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<N, C: Ord + Copy + Add<Output = C> + Default> Ord for Pending<N, C> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost_and_heuristic.cmp(&self.cost_and_heuristic)
        }
    }

    let mut pending = BinaryHeap::new();
    pending.push(Pending {
        cost: C::default(),
        cost_and_heuristic: heuristic(&start),
        node: start,
        previous: None,
    });
    let mut visited = AHashMap::<N, (C, Option<N>)>::new();
    let mut next_nodes = Vec::new();
    while let Some(entry) = pending.pop() {
        if is_target(&entry.node) {
            let total_cost = entry.cost;
            // Reconstruct path
            let mut path = Vec::new();
            path.push((entry.node, total_cost));
            let mut previous = entry.previous;
            while let Some(node) = previous {
                let cost;
                (cost, previous) = visited.remove(&node).unwrap();
                path.push((node, cost));
            }

            path.reverse();
            return Some(AStarInfo { total_cost, path });
        }
        match visited.entry(entry.node.clone()) {
            Entry::Occupied(mut previously_visited) => {
                let previous = previously_visited.get_mut();
                if previous.0 <= entry.cost {
                    continue;
                }
                previous.0 = entry.cost;
                previous.1 = entry.previous.clone();
            }
            Entry::Vacant(slot) => {
                slot.insert((entry.cost, entry.previous.clone()));
            }
        }
        next(&entry.node, &mut next_nodes);
        for (next_node, next_cost) in next_nodes.drain(..) {
            let cost = entry.cost + next_cost;
            pending.push(Pending {
                cost,
                cost_and_heuristic: cost + heuristic(&next_node),
                node: next_node,
                previous: Some(entry.node.clone()),
            });
        }
    }
    None
}

pub fn astar_no_path<N, C, FN, FH, FC>(
    start: N,
    mut next: FN,
    mut heuristic: FH,
    mut is_target: FC,
) -> Option<C>
where
    N: Clone + Hash + Eq,
    C: Ord + Copy + Add<Output = C> + Default,
    FN: FnMut(&N, &mut Vec<(N, C)>),
    FH: FnMut(&N) -> C,
    FC: FnMut(&N) -> bool,
{
    struct Pending<N, C: Ord + Copy + Add<Output = C> + Default> {
        cost: C,
        cost_and_heuristic: C,
        node: N,
    }

    impl<N, C: Ord + Copy + Add<Output = C> + Default> PartialEq for Pending<N, C> {
        fn eq(&self, other: &Self) -> bool {
            self.cost_and_heuristic == other.cost_and_heuristic
        }
    }
    impl<N, C: Ord + Copy + Add<Output = C> + Default> Eq for Pending<N, C> {}
    impl<N, C: Ord + Copy + Add<Output = C> + Default> PartialOrd for Pending<N, C> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<N, C: Ord + Copy + Add<Output = C> + Default> Ord for Pending<N, C> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost_and_heuristic.cmp(&self.cost_and_heuristic)
        }
    }

    let mut pending = BinaryHeap::new();
    pending.push(Pending {
        cost: C::default(),
        cost_and_heuristic: heuristic(&start),
        node: start,
    });
    let mut visited = AHashMap::<N, C>::new();
    let mut next_nodes = Vec::new();
    while let Some(entry) = pending.pop() {
        if is_target(&entry.node) {
            return Some(entry.cost);
        }
        match visited.entry(entry.node.clone()) {
            Entry::Occupied(mut previously_visited) => {
                let previous = previously_visited.get_mut();
                if *previous <= entry.cost {
                    continue;
                }
                *previous = entry.cost;
            }
            Entry::Vacant(slot) => {
                slot.insert(entry.cost);
            }
        }
        next(&entry.node, &mut next_nodes);
        for (next_node, next_cost) in next_nodes.drain(..) {
            let cost = entry.cost + next_cost;
            pending.push(Pending {
                cost,
                cost_and_heuristic: cost + heuristic(&next_node),
                node: next_node,
            });
        }
    }
    None
}
