use std::collections::{HashMap, HashSet, VecDeque};
use plotters::{prelude::*, style::full_palette::BLUEGREY};

//This is the function that calculating sixth degree path and theiur statistic
pub fn calculate_sixth_degree_stats(graph: &HashMap<u32, HashSet<u32>>) -> (usize,Vec<usize>, f64, f64, f64,f64) {
    let mut sixth_degree_paths_count = 0;
    let mut sixth_degree_per_vertex: Vec<usize> = Vec::new(); 
    let vertices_count = graph.keys().len();
    
    for &vertex in graph.keys() {
        let paths = bfs_sixth_degree_paths(graph, vertex);
        sixth_degree_paths_count += paths.len();
        sixth_degree_per_vertex.push(paths.len()); 
    }
    
    let mean_sixth_degree = sixth_degree_paths_count as f64 / vertices_count as f64;
    let variance = calculate_variance(&sixth_degree_per_vertex, mean_sixth_degree);
    let total_possible_pairs = vertices_count * (vertices_count - 1) / 2;
    let std_deviation = calculate_variance(&sixth_degree_per_vertex, mean_sixth_degree).sqrt();
    let proportion_of_sixth_degree_pairs = sixth_degree_paths_count as f64 / total_possible_pairs as f64;
    
    (sixth_degree_paths_count,sixth_degree_per_vertex, mean_sixth_degree, proportion_of_sixth_degree_pairs, variance, std_deviation )
}
//This is a function that is set to find variance for the sixth degree path, also it can find std deviation by using sqrt
fn calculate_variance(sixth_degree_counts: &Vec<usize>, mean_sixth_degree: f64) -> f64 {
    let variance: f64 = sixth_degree_counts.iter().map(|&count| {
        let diff = count as f64 - mean_sixth_degree;
        diff * diff
    }).sum::<f64>() / sixth_degree_counts.len() as f64;
    
    variance
}
//bfs to find which id1 will reach id2 with exactly six steps, it will be call on the top when we calculating stat
pub fn bfs_sixth_degree_paths(graph: &HashMap<u32, HashSet<u32>>, start: u32) -> HashSet<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0));
    let mut sixth_degree_vertices = HashSet::new();

    while let Some((current_vertex, degree)) = queue.pop_front() {
        if degree == 6 {
            sixth_degree_vertices.insert(current_vertex);
            continue;
        }

        if let Some(neighbors) = graph.get(&current_vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, degree + 1));
                }
            }
        }
    }

    sixth_degree_vertices
}
// function that find avg distance for each id and their combination, also it's set in a direct graph
pub fn average_distance(graph: &HashMap<u32, HashSet<u32>>) -> f64 {
    let mut total_distance = 0;
    let mut count = 0;

    for &vertex in graph.keys() {
        let distances = bfs_directed(graph, vertex);
        for (&target_vertex, &distance) in distances.iter() {
            if vertex != target_vertex {
                total_distance += distance;
                count += 1;
            }
        }
    }

    if count > 0 {
        total_distance as f64 / count as f64
    } else {
        0.0 
    }
}
//bfs function that is use to be called in avg distance function
//it will find the bfs with no limitstion of steps
fn bfs_directed(graph: &HashMap<u32, HashSet<u32>>, start: u32) -> HashMap<u32, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    let current_distance = *distances.get(&current).unwrap_or(&0);
                    distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distances.remove(&start);
    distances
}

//plotting an image for with x is sum for sixth degree path after exploring each combination
//y is the number of id(frequency) that having this number of x of sixth degree
pub fn plot_sixth_degree_distribution(output_file: &str, sixth_degree_per_vertex: &Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(output_file, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let max_path_count = *sixth_degree_per_vertex.iter().max().unwrap() as i32;
    
    let mut frequencies = vec![0; (max_path_count + 1) as usize]; 
    for &count in sixth_degree_per_vertex {
        frequencies[count] += 1; 
    }

    let histogram_data: Vec<(i32, i32)> = frequencies.into_iter().enumerate()
        .map(|(count, freq)| (count as i32, freq as i32)) 
        .collect();

    let max_frequency = histogram_data.iter().map(|&(_, freq)| freq).max().unwrap();
    let x_axis_min = 0;    
    let x_axis_max = 3300;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Sixth Degree Path Distribution", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_axis_min..x_axis_max, 0..(max_frequency + 1))?; 

    chart.configure_mesh().draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUEGREY.mix(0.5).filled())
            .data(histogram_data.iter().map(|&(x, y)| (x, y))),
    )?;

    root_area.present()?;

    Ok(())
}
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    fn create_linear_graph_with_sixth_degree() -> HashMap<u32, HashSet<u32>> {
        let mut graph = HashMap::new();
        for i in 1..7 {
            graph.insert(i, HashSet::from([i + 1]));
        }
        graph.insert(7, HashSet::new());
        graph
    }

    #[test]
    fn test_sixth_degree_paths_count() {
        let graph = create_linear_graph_with_sixth_degree();
        let expected_count = 1;
        let (count, _, _, _, _, _) = calculate_sixth_degree_stats(&graph);
        assert_eq!(count, expected_count, "Expected one sixth-degree path");
    }

    #[test]
    fn test_average_sixth_degree() {
        let graph = create_linear_graph_with_sixth_degree();
        let expected_average = 1.0 / graph.len() as f64;
        let (_, _, average, _, _, _) = calculate_sixth_degree_stats(&graph);
        assert!((average - expected_average).abs() < f64::EPSILON, "Average sixth degree did not match expected value");
    }

}