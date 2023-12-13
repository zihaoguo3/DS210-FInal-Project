use std::collections::{HashMap, HashSet};
// using jaccard similarity to find the most similar pair and dissimilar pair
//calling the function downstair
//use forloop to discover all of the possible combination
pub fn find_extreme_pairs(graph: &HashMap<u32, HashSet<u32>>) -> ((u32, u32, f64), (u32, u32, f64)) {
    let mut most_similar = (0, 0, 0.0); 
    let mut most_dissimilar = (0, 0, 1.0); 

    let keys: Vec<&u32> = graph.keys().collect();
    for i in 0..keys.len() {
        for j in i + 1..keys.len() {
            let vertex1 = keys[i];
            let vertex2 = keys[j];
            let jaccard_coefficient = jaccard_similarity(
                graph.get(vertex1).unwrap(),
                graph.get(vertex2).unwrap(),
            );

            if jaccard_coefficient > most_similar.2 {
                most_similar = (*vertex1, *vertex2, jaccard_coefficient);
            }

            if jaccard_coefficient < most_dissimilar.2 && jaccard_coefficient > 0.0 {
                most_dissimilar = (*vertex1, *vertex2, jaccard_coefficient);
            }
        }
    }

    (most_similar, most_dissimilar)
}
//the range of jaccard similarity is 0-1. 0 means the most disimilar and 1 means most similiar
//using total of a and b devide by total of a or b
fn jaccard_similarity(set1: &HashSet<u32>, set2: &HashSet<u32>) -> f64 {
    let intersection_size = set1.intersection(set2).count() as f64;
    let union_size = set1.union(set2).count() as f64;
    intersection_size / union_size
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard_similarity() {
        let set1: HashSet<u32> = HashSet::from([1, 2, 3]);
        let set2: HashSet<u32> = HashSet::from([2, 3, 4]);

        let jaccard = jaccard_similarity(&set1, &set2);
        assert_eq!(jaccard, 0.5);
    }
}
