use petgraph::graph::{DiGraph};
use petgraph::dot::{Dot, Config};

use johnson::johnson;

fn sample_johnson() {

	// Source:
	//		https://algs4.cs.princeton.edu/44sp/
	// 		Graph image: https://algs4.cs.princeton.edu/44sp/images/shortest-path.png

	let mut graph = DiGraph::<&str,f32>::new();
	let zero = graph.add_node("0");
	let one = graph.add_node("1");
	let two = graph.add_node("2");
	let three = graph.add_node("3");
	let four = graph.add_node("4");
	let five = graph.add_node("5");
	let six = graph.add_node("6");
	let seven = graph.add_node("7");

	graph.extend_with_edges(&[
		(zero, two,  0.26), (zero,four, 0.38),
		(one,  three,0.29),
		(two,  seven,0.34),
		(three,six,  0.52),
		(four, five, 0.35), (four, seven,0.37),
		(five, four, 0.35), (five, seven,0.28), (five, one,  0.32),
		(six,  two,  0.40), (six,  zero, 0.58), (six,  four, 0.93),
		(seven,five, 0.28), (seven,three,0.39),
		
	]); 
	let costs = johnson(&graph,zero,Some(six)).unwrap();

	// Outputs 1.51
	println!("Cost for going from {} to {} = {}",
			graph.node_weight(zero).unwrap(),
			graph.node_weight(six).unwrap(),
			costs.get(&six).unwrap()
	);

	println!("Dot file, for generating PNG:");
	println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
fn main() {

	sample_johnson();
}
