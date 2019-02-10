use crate::algebra::Position;
use crate::map::GameMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;

#[derive(Debug)]
pub struct Edge {
    pub position: Position,
    pub cost: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    pub position: Position,
    pub cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path(
    graph: &HashMap<Position, Vec<Edge>>,
    start: Position,
    goal: Position,
) -> HashMap<Position, Position> {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        position: start,
        cost: 0,
    });
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some(State { cost, position }) = frontier.pop() {
        if position == goal {
            return came_from;
        }
        for edge in graph.get(&position).unwrap().iter() {
            let new_cost = cost + edge.cost;
            let next = State {
                cost: new_cost,
                position: edge.position,
            };
            if new_cost
                < *cost_so_far
                    .get(&next.position)
                    .unwrap_or(&usize::max_value())
            {
                cost_so_far.insert(next.position, new_cost);
                frontier.push(next);
                came_from.insert(next.position, position);
            }
        }
    }
    unreachable!();
    return came_from;
}

pub fn reconstruct_path(
    came_from: HashMap<Position, Position>,
    start: Position,
    goal: Position,
) -> Vec<Position> {
    let mut current = goal;
    let mut path = vec![];
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }
    path.push(start);
    path.reverse();
    return path;
}
