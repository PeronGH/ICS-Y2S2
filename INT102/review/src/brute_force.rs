use std::collections::{HashSet, VecDeque};

use petgraph::prelude::*;

pub trait BruteForceSorting<T: Ord> {
    fn selection_sort(&mut self) -> &mut [T];
    fn bubble_sort(&mut self) -> &mut [T];
    fn insertion_sort(&mut self) -> &mut [T];
}

impl<T: Ord> BruteForceSorting<T> for [T] {
    fn selection_sort(&mut self) -> &mut [T] {
        selection_sort(self);
        self
    }

    fn bubble_sort(&mut self) -> &mut [T] {
        bubble_sort(self);
        self
    }

    fn insertion_sort(&mut self) -> &mut [T] {
        insertion_sort(self);
        self
    }
}

fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        if let Some((min_i, _)) = arr.iter().enumerate().skip(i).min_by_key(|(_, e)| *e) {
            arr.swap(i, min_i)
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn linear_search<T: PartialEq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .enumerate()
        .find_map(|(i, current)| if current == needle { Some(i) } else { None })
}

pub fn bfs<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> HashSet<NodeIndex> {
    let mut deque = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    print!("{:?} ", start);

    while let Some(node) = deque.pop_front() {
        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                print!("{:?} ", neighbor);
                deque.push_back(neighbor);
            }
        }
    }

    println!();
    visited
}

pub fn dfs<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> HashSet<NodeIndex> {
    let mut stack = vec![start];
    let mut visited = HashSet::new();

    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            visited.insert(node);
            print!("{:?} ", node);
            stack.extend(graph.neighbors(node));
        }
    }

    println!();
    visited
}
