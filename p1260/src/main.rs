use std::collections::VecDeque;
use std::io::{self, Read};

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>){
    visited[v] = true;
    print!("{}", v);

    for &next in &graph[v]{ // Q. Why ref char attached at $next?
        if !visited[next] {
            dfs(next, graph, visited);
        }
    }
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>){
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(v) = queue.pop_front(){
        print!("{}", v);

        for &next in &graph[v]{
            if !visited[next]{
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let v: usize = iter.next().unwrap().parse().unwrap();

    let mut graph = vec![vec![]; n+1];

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 1..=n {
        graph[i].sort();
    }
    
    //dfs
    let mut visited = vec![false; n+1];
    dfs(v, &graph, &mut visited);
    println!();

    //bfs
    bfs(v, &graph);

}
