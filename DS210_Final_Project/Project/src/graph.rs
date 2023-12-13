use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
// store the two entry in the sam row with different column in to the vector
// from is index 0 (left column), to is index 1 (right column)
pub fn read_graph<P>(filename: P) -> io::Result<Vec<(u32, u32)>>
where
    P: AsRef<Path>,
    {

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let from = parts[0].parse::<u32>().unwrap();
            let to = parts[1].parse::<u32>().unwrap();
            edges.push((from, to));
        }
    }

    Ok(edges)
}
// build a hashmap and store from and to into hashmap to build graph
// hashset is containing the vertices that are directly connected to the key vertex by an edge.
pub fn build_graph(edges: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut graph = HashMap::new();

    for &(from, to) in edges {
        graph.entry(from).or_insert_with(HashSet::new).insert(to);
        graph.entry(to).or_insert_with(HashSet::new).insert(from);
    }

    graph
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_build_graph() {
        let edges = vec![(1, 2), (2, 3), (3, 1)];

        let graph = build_graph(&edges);

        assert!(graph.get(&1).unwrap().contains(&2));
        assert!(graph.get(&2).unwrap().contains(&3));
        assert!(graph.get(&3).unwrap().contains(&1));
    }
}

