use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
};
/*
You are given an array of positive integers arr. Perform some operations (possibly none) on arr so that it satisfies these conditions:

    The value of the first element in arr must be 1.
    The absolute difference between any 2 adjacent elements must be less than or equal to 1. In other words, abs(arr[i] - arr[i - 1]) <= 1 for each i where 1 <= i < arr.length (0-indexed). abs(x) is the absolute value of x.

There are 2 types of operations that you can perform any number of times:

    Decrease the value of any element of arr to a smaller positive integer.
    Rearrange the elements of arr to be in any order.

Return the maximum possible value of an element in arr after performing the operations to satisfy the conditions.
*/
pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();
    let mut max = 0;
    for elem in arr {
        if elem > max {
            max += 1;
        }
    }
    max
}

/*
There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1. The edges of the graph are initially represented by the given array edges where edges[i] = [fromi, toi, edgeCosti] meaning that there is an edge from fromi to toi with the cost edgeCosti.

Implement the Graph class:

    Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
    addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
    int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.

Constraints:
    1 <= n <= 100
    0 <= edges.length <= n * (n - 1)
    edges[i].length == edge.length == 3
    0 <= fromi, toi, from, to, node1, node2 <= n - 1
    1 <= edgeCosti, edgeCost <= 106
    There are no repeated edges and no self-loops in the graph at any point.
    At most 100 calls will be made for addEdge.
    At most 100 calls will be made for shortestPath.


*/
pub struct Graph {
    pub adjacency: HashMap<i32, Vec<Node>>,
    n: i32,
}

impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Graph {
            n,
            adjacency: HashMap::new(),
        };

        // build graph stored as adjacency map
        for edge in edges {
            graph.add_edge(edge);
        }

        graph
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) {
        self.adjacency
            .entry(edge[0])
            .and_modify(|vec| {
                vec.push(Node {
                    node: edge[1],
                    cost: edge[2],
                });
            })
            .or_insert(vec![Node {
                node: edge[1],
                cost: edge[2],
            }]);
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        // Djikstra's algorithm, with the BTreeSet sorted by current costs,
        // and the nodes as indexes to the nodes and visted vectors
        //        let nodes: Vec<Node> = Vec::with_capacity(self.n.try_into().unwrap());
        let mut visited: Vec<bool> = vec![false; self.n.try_into().unwrap()];
        let mut costs: BTreeSet<Node> = BTreeSet::new();

        // set the first node equal to cost of 0, and mark the first node as visited
        let mut current_node = Some(Node {
            cost: 0,
            node: node1,
        });
        visited[TryInto::<usize>::try_into(node1).unwrap()] = true;

        while let Some(cur_node) = current_node.clone() {
            // set current node to visited
            println!("visiting {}", cur_node.node);
            visited[TryInto::<usize>::try_into(cur_node.node).unwrap()] = true;
            if cur_node.node == node2 {
                return cur_node.cost;
            }

            // iterate through all nodes adjascent to cur_node updating costs as needed
            let adjascent_vec = self.adjacency.get(&cur_node.node);
            if adjascent_vec.is_some() {
                for adjascent_node in adjascent_vec.unwrap() {
                    // check if this node has already been visited
                    if !visited[TryInto::<usize>::try_into(adjascent_node.node).unwrap()] {
                        let mut this_cost = 0;
                        let updated: Option<i32> = match costs.get(adjascent_node) {
                            Some(node) => {
                                this_cost = cur_node.cost + adjascent_node.cost;
                                // found a cheaper path than previously found
                                if this_cost < node.cost {
                                    // update the value
                                    Some(node.node)
                                } else {
                                    // stored path was cheaper, nothing to do
                                    None
                                }
                            }
                            // No stored cost found, insert the current cost
                            None => {
                                costs.insert(Node {
                                    cost: cur_node.cost + adjascent_node.cost,
                                    node: adjascent_node.node,
                                });
                                None
                            }
                        };

                        if let Some(node) = updated {
                            println!(
                                "better path to node {} discovered, cost {}",
                                node, this_cost
                            );
                            costs.replace(Node {
                                cost: this_cost,
                                node,
                            });
                        }
                    }
                }
            }

            let first;
            {
                first = costs.first().map(|opt| opt.clone());
            }
            current_node = match first {
                Some(node) => costs.take(&node),
                None => None,
            }
        }

        -1
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub cost: i32,
    pub node: i32,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.node.partial_cmp(&other.node)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.cmp(&other.node)
    }
}

/*
There is an integer array nums that consists of n unique elements, but you have forgotten it. However, you do remember every pair of adjacent elements in nums.

You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums.

It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order.

Return the original array nums. If there are multiple solutions, return any of them.

Constraints:
    nums.length == n
    adjacentPairs.length == n - 1
    adjacentPairs[i].length == 2
    2 <= n <= 105
    -105 <= nums[i], ui, vi <= 105
    There exists some nums that has adjacentPairs as its pairs.

*/
pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adjacency: HashMap<i32, Vec<i32>> = HashMap::new();
    let one = adjacent_pairs[0][0];
    let two = adjacent_pairs[0][1];

    for pair in adjacent_pairs.into_iter().skip(1) {
        adjacency
            .entry(pair[0])
            .and_modify(|existing| existing.push(pair[1]))
            .or_insert(vec![pair[1]]);
        adjacency
            .entry(pair[1])
            .and_modify(|existing| existing.push(pair[0]))
            .or_insert(vec![pair[0]]);
    }

    let mut forward = Vec::new();
    let mut backward = Vec::new();

    forward.push(one);
    restore_array_follow_links(&mut forward, &mut adjacency, one);

    backward.push(two);
    restore_array_follow_links(&mut backward, &mut adjacency, two);

    backward
        .into_iter()
        .rev()
        .chain(forward.into_iter())
        .collect()
}

fn restore_array_follow_links(
    restored: &mut Vec<i32>,
    adjacency: &mut std::collections::HashMap<i32, Vec<i32>>,
    start_point: i32,
) {
    let mut cursor = start_point;
    while let Some(mut adjascent) = adjacency.remove(&cursor) {
        // store the next element on the reconstruction array
        // or break if we hit an empty array
        let next = match adjascent.pop() {
            Some(next) => next,
            None => break,
        };
        restored.push(next);

        // remove the reverse of the link we just followed
        let list = adjacency.get_mut(&next).unwrap();
        if list[0] == cursor {
            list.remove(0);
        } else {
            list.pop();
        }

        // next
        cursor = next;
    }
}

pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut times: Vec<i32> = dist
        .into_iter()
        .zip(speed.into_iter())
        // ceil division
        .map(|(dist, speed)| -> i32 { (dist + speed - 1) / speed })
        .collect();

    println!("{:#?}", times);
    times.sort_unstable();

    let mut cur_time: i32 = 0;
    for time in times {
        if time <= cur_time {
            break;
        }
        cur_time += 1;
    }

    std::cmp::max(cur_time, 1)
}

/*
Design a system that manages the reservation state of n seats that are numbered from 1 to n.

Implement the SeatManager class:

    SeatManager(int n) Initializes a SeatManager object that will manage n seats numbered from 1 to n. All seats are initially available.
    int reserve() Fetches the smallest-numbered unreserved seat, reserves it, and returns its number.
    void unreserve(int seatNumber) Unreserves the seat with the given seatNumber.

*/
pub struct SeatManager {
    priority_queue: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        let priority_queue: BinaryHeap<Reverse<i32>> = (1..=n).rev().map(|i| Reverse(i)).collect();
        SeatManager { priority_queue }
    }

    pub fn reserve(&mut self) -> i32 {
        self.priority_queue.pop().unwrap().0
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        self.priority_queue.push(Reverse(seat_number))
    }
}

/*
We are given an array asteroids of integers representing asteroids in a row.

For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.

Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
*/
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut moving_left = Vec::new();
    let mut moving_right = Vec::new();

    for asteroid in asteroids.into_iter() {
        // asteroid is moving left
        if asteroid < 0 {
            loop {
                match moving_right.last() {
                    Some(top) => {
                        // asteroid destroys top of moving_right
                        if asteroid.abs() > *top {
                            moving_right.pop();
                        }
                        // both asteroids are destroyed
                        else if asteroid.abs() == *top {
                            moving_right.pop();
                            break;
                        }
                        // asteroid is destroyed
                        else {
                            break;
                        }
                    }
                    // No asteroids moving right
                    None => {
                        moving_left.push(asteroid);
                        break;
                    }
                }
            }
        }
        // asteroid is moving right
        else {
            moving_right.push(asteroid)
        }
    }

    moving_left.append(&mut moving_right);
    moving_left
}

/*
Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
*/
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut current = &intervals[0];
    let mut num_removals: i32 = 0;

    for interval in intervals.iter().skip(1) {
        // interval conflicts
        if interval[0] >= current[0] && interval[0] < current[1] {
            // set current equal to whichever interval ends the soonest
            if interval[1] < current[1] {
                current = interval;
            }
            num_removals += 1;
        }
        // interval does not conflict
        else {
            current = interval;
        }
    }

    num_removals
}

/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    nums.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut result: Vec<i32> = Vec::new();

    let mut lower = 0;
    let mut upper = nums.len() - 1;

    loop {
        let test = nums[lower].1 + nums[upper].1;
        if test == target {
            result.push(nums[lower].0.try_into().unwrap());
            result.push(nums[upper].0.try_into().unwrap());
            break;
        } else if test < target {
            lower += 1;
        } else {
            upper -= 1;
        }

        assert!(lower < upper);
    }

    result
}
