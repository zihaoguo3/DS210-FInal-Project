mod graph;
mod separation;
mod similarity;

use std::io;
use crate::separation::plot_sixth_degree_distribution;
use crate::separation::average_distance;

//call all of the function that will provide answer and in different module to make sure they are in scope and print out the result
fn main() -> io::Result<()> {
    let path = "p2p.txt"; 
    let edges = graph::read_graph(path)?;
    let graph = graph::build_graph(&edges);
    let (sixth_degree_paths_count,sixth_degree_per_vertex, mean_sixth_degree, proportion_of_sixth_degree_pairs, variance, std_deviation) = separation::calculate_sixth_degree_stats(&graph);

    println!("Total sixth-degree paths count: {}", sixth_degree_paths_count);
    println!("Mean sixth-degree connections per vertex: {}", mean_sixth_degree);
    println!("Proportion of vertex pairs with a sixth-degree connection: {}", proportion_of_sixth_degree_pairs);
    println!("Variance of sixth-degree connections per vertex: {}", variance);
    println!("Standard deviation is:{} ",std_deviation);

    let avg_distance = average_distance(&graph);
    println!("The average distance between pairs of vertices in the graph is: {}", avg_distance);


    let (most_similar, most_dissimilar) = similarity::find_extreme_pairs(&graph);
    
    println!("Most Similar Pair: ({}, {}) with Jaccard Coefficient: {}", most_similar.0, most_similar.1, most_similar.2);
    println!("Most Dissimilar Pair: ({}, {}) with Jaccard Coefficient: {}", most_dissimilar.0, most_dissimilar.1, most_dissimilar.2);
    //make a if statement, to make sure that will pops up error if it can''t draw the graph
    if let Err(e) = plot_sixth_degree_distribution("sixth_degree_distribution.png", &sixth_degree_per_vertex) {
        eprintln!("Error plotting sixth degree distribution: {}", e);
    }

    Ok(())
}

