use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

// Leetcode 2391. Minimum Amount of Time to Collect Garbage
pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    // represents time for each truck
    let mut m_truck = 0;
    let mut p_truck = 0;
    let mut g_truck = 0;

    // represents potential travel time (conditional
    // on finding garbage further along)
    let mut tm = 0;
    let mut tp = 0;
    let mut tg = 0;

    for (garb, travel) in garbage.into_iter().zip((0..1).chain(travel.into_iter())) {
        tm += travel;
        tp += travel;
        tg += travel;

        for char in garb.chars() {
            match char {
                'M' => {
                    m_truck += 1;
                    m_truck += tm;
                    tm = 0;
                }
                'P' => {
                    p_truck += 1;
                    p_truck += tp;
                    tp = 0;
                }
                'G' => {
                    g_truck += 1;
                    g_truck += tg;
                    tg = 0;
                }
                _ => panic!("invalid input"),
            }
        }
    }

    m_truck + p_truck + g_truck
}

// Leetcode 26. Remove Duplicates from Sorted Array
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last_seen = -101;
    let mut k = 0;

    // use retain to efficiently filter out duplicates
    nums.retain(|elem| {
        if *elem == last_seen {
            false
        } else {
            k += 1;
            last_seen = *elem;
            true
        }
    });
    k
}

// Leetcode 1887. Reduction Operations to Make the Array Elements Equal
pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
    // sort the array
    nums.sort_unstable();

    // iterate over the array and count the operations
    let mut operations = 0;
    let mut num_operations = 0;
    let mut last_elem = nums[0];
    for num in nums.into_iter().skip(1) {
        if num == last_elem {
            operations += num_operations;
        } else {
            num_operations += 1;
            operations += num_operations;
            last_elem = num;
        }
    }

    operations
}

/*
The frequency of an element is the number of times it occurs in an array.

You are given an integer array nums and an integer k. In one operation, you can choose an index of nums and increment the element at that index by 1.

Return the maximum possible frequency of an element after performing at most k operations.
*/
pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut max_freq = 1;
    for i in (1..nums.len()).rev() {
        let current_freq = max_frequency_index(&mut nums, k, i);

        if current_freq > max_freq {
            max_freq = current_freq;
        }

        if max_freq > i as i32 {
            return max_freq;
        }
    }

    max_freq
}

pub fn max_frequency_index(nums: &mut Vec<i32>, k: i32, i: usize) -> i32 {
    // find the max frequency achievable from the current index
    let mut j = i - 1;
    let mut freq = 1;
    let mut current_k = 0;

    loop {
        current_k += nums[i] - nums[j];

        if current_k <= k {
            // current_k within limits, increment freq
            freq += 1;
        } else {
            // current_k exceeded limits, return freq
            return freq;
        }

        if j == 0 {
            // reached beginning of list, nothing more to do
            return freq;
        }

        j -= 1;
    }
}

/*
The pair sum of a pair (a,b) is equal to a + b. The maximum pair sum is the largest pair sum in a list of pairs.

    For example, if we have pairs (1,5), (2,3), and (4,4), the maximum pair sum would be max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8.

Given an array nums of even length n, pair up the elements of nums into n / 2 pairs such that:

    Each element of nums is in exactly one pair, and
    The maximum pair sum is minimized.

Return the minimized maximum pair sum after optimally pairing up the elements.
*/
pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    // sort the array
    nums.sort_unstable();

    let mut front = 0;
    let mut back = nums.len() - 1;
    let mut max = 0;
    // The pairs are the largest and smallest, second largest and second smallest, etc
    while front < back {
        let pair = nums[front] + nums[back];

        if pair > max {
            max = pair;
        }

        // update indexes
        front += 1;
        back -= 1;
    }

    max
}

/*
Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length nthat does not appear in nums. If there are multiple answers, you may return any of them.
*/
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    nums.into_iter()
        .enumerate()
        .map(|(i, bin_string)| -> char {
            match bin_string.as_bytes()[i] as char {
                '0' => '1',
                '1' => '0',
                _ => panic!("invalid input, string does not consist of only 1s and 0s"),
            }
        })
        .collect()
}

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
                    cumulative_cost: None,
                    visited: false,
                });
            })
            .or_insert(vec![Node {
                node: edge[1],
                cost: edge[2],
                cumulative_cost: None,
                visited: false,
            }]);
    }

    pub fn shortest_path(&mut self, node1: i32, node2: i32) -> i32 {
        // Djikstra's algorithm

        // initialize algorithm
        let mut costs: Vec<Reverse<CostNode>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.n as usize];
        costs.push(Reverse(CostNode {
            cumulative_cost: 0,
            node: node1,
        }));

        // run the algorithm
        while let Some(Reverse(node)) = costs.pop() {
            // found endpoint
            if node.node == node2 {
                self.reset_djikstras();
                return node.cumulative_cost;
            }
            // already visited this node
            else if visited[node.node as usize] {
                continue;
            }

            // mark current node as visited
            visited[node.node as usize] = true;

            if let Some(vec) = self.adjacency.get_mut(&node.node) {
                for adj_node in vec.iter_mut() {
                    // already visited
                    if visited[adj_node.node as usize] {
                        continue;
                    }

                    // check if new path or cheaper path
                    if let Some(cumulative_cost) = adj_node.cumulative_cost {
                        // cheaper path found, update path, and add it to cost vector
                        if node.cumulative_cost + adj_node.cost < cumulative_cost {
                            adj_node.cumulative_cost = Some(node.cumulative_cost + adj_node.cost);
                            costs.push(Reverse(CostNode::new(
                                adj_node.node,
                                adj_node.cumulative_cost.unwrap(),
                            )));
                        }
                    }
                    // no existing path, create this one and add it to costs
                    else {
                        adj_node.cumulative_cost = Some(node.cumulative_cost + adj_node.cost);
                        costs.push(Reverse(CostNode::new(
                            adj_node.node,
                            adj_node.cumulative_cost.unwrap(),
                        )));
                    }
                }
            }

            costs.sort_unstable();
        }

        self.reset_djikstras();
        // did not find endpoint, no path available
        -1
    }

    fn reset_djikstras(&mut self) {
        for (_key, val) in self.adjacency.iter_mut() {
            for entry in val {
                entry.cumulative_cost = None;
                entry.visited = false;
            }
        }
    }
}

// equality, ordering, and hashing by node
#[derive(Clone, Debug)]
pub struct Node {
    pub cost: i32,
    pub node: i32,
    pub cumulative_cost: Option<i32>,
    pub visited: bool,
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

// equality and ordering by cost
pub struct CostNode {
    pub cumulative_cost: i32,
    pub node: i32,
}

impl PartialEq for CostNode {
    fn eq(&self, other: &Self) -> bool {
        self.cumulative_cost.eq(&other.cumulative_cost)
    }
}

impl PartialOrd for CostNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cumulative_cost.partial_cmp(&other.cumulative_cost)
    }
}

impl Ord for CostNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cumulative_cost.cmp(&other.cumulative_cost)
    }
}

impl Eq for CostNode {}

impl CostNode {
    pub fn new(node: i32, cumulative_cost: i32) -> Self {
        CostNode {
            cumulative_cost,
            node,
        }
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

#[cfg(test)]
mod test {
    #[test]
    fn max_frequency_case1() {
        let nums = vec![1, 2, 4];
        let k = 5;

        assert_eq!(3, super::max_frequency(nums, k));
    }

    #[test]
    fn max_frequency_case2() {
        let nums = vec![1, 4, 8, 13];
        let k = 5;

        assert_eq!(2, super::max_frequency(nums, k));
    }

    #[test]
    fn max_frequency_case3() {
        let nums = vec![3, 9, 6];
        let k = 2;

        assert_eq!(1, super::max_frequency(nums, k));
    }

    #[test]
    fn max_frequency_case4() {
        let nums = vec![3, 9, 6];
        let k = 2;

        assert_eq!(1, super::max_frequency(nums, k));
    }

    #[test]
    fn reduction_operations_case1() {
        let nums = vec![5, 1, 3];

        assert_eq!(3, super::reduction_operations(nums));
    }

    #[test]
    fn reduction_operations_case2() {
        let nums = vec![1, 1, 1];

        assert_eq!(0, super::reduction_operations(nums));
    }

    #[test]
    fn reduction_operations_case3() {
        let nums = vec![1, 1, 2, 2, 3];

        assert_eq!(4, super::reduction_operations(nums));
    }

    #[test]
    fn remove_duplicates_case1() {
        let mut nums = vec![1, 1, 2];

        assert_eq!(2, super::remove_duplicates(&mut nums));
        assert_eq!(1, nums[0]);
        assert_eq!(2, nums[1]);
    }

    #[test]
    fn remove_duplicates_case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        assert_eq!(5, super::remove_duplicates(&mut nums));
        assert_eq!(0, nums[0]);
        assert_eq!(1, nums[1]);
        assert_eq!(2, nums[2]);
        assert_eq!(3, nums[3]);
        assert_eq!(4, nums[4]);
    }

    #[test]
    fn garbage_collection_case1() {
        let garbage = vec![
            "G".to_string(),
            "P".to_string(),
            "GP".to_string(),
            "GG".to_string(),
        ];
        let travel = vec![2, 4, 3];

        assert_eq!(21, super::garbage_collection(garbage, travel));
    }

    #[test]
    fn garbage_collection_case2() {
        let garbage = vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()];
        let travel = vec![3, 10];

        assert_eq!(37, super::garbage_collection(garbage, travel));
    }
}
