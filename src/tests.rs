#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use petgraph::graph::{NodeIndex, DiGraph};
	use crate::johnson;

    #[test]
	fn test_case1() {
	
		// Source: 
		//		https://www.geeksforgeeks.org/johnsons-algorithm-for-all-pairs-shortest-paths-implementation/

		let mut graph = DiGraph::<&str,f32>::new();
		let zero = graph.add_node("0");
		let one = graph.add_node("1");
		let two = graph.add_node("2");
		let three = graph.add_node("3");
		graph.extend_with_edges(&[
			(zero, one, -5.0), (zero, two, 2.0), (zero, three, 3.0),
			(one,two,4.0),
			(two,three,1.0),
		]);

		// From specific source vertices to all the other (destination) vertices
		// From vertex '0'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(zero,0.),
			(one,0.),
			(two,0.),
			(three,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,zero,None).unwrap();
		assert_eq!(costs,expected_costs);

		// From vertex '1'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(one,0.),
			(two,0.),
			(three,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,one,None).unwrap();
		assert_eq!(costs,expected_costs);
		
		// From vertex '2'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(two,0.),
			(three,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,two,None).unwrap();
		assert_eq!(costs,expected_costs);

		// From vertex '3'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(three,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,three,None).unwrap();
		assert_eq!(costs,expected_costs);

		// Picking some specific routes
		// '0' -> '2'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(two,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,zero,Some(two)).unwrap();
		assert_eq!(costs,expected_costs);
		
		// Picking some specific routes
		// '0' -> '3'
		let expected_costs: HashMap<NodeIndex, f32> = HashMap::new();	// non-existent path
		let costs = johnson(&graph,three,Some(zero)).unwrap();
		assert_eq!(costs,expected_costs);

		// Picking some specific routes
		// '0' -> '3'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(three,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,zero,Some(three)).unwrap();
		assert_eq!(costs,expected_costs);

	}

    #[test]
	fn test_case2() {
		// Source: 
		//		https://www.javatpoint.com/johnsons-algorithm	

		let mut graph = DiGraph::<&str,f32>::new();
		let a = graph.add_node("a");
		let b = graph.add_node("b");
		let c = graph.add_node("c");
		let d = graph.add_node("d");
		graph.extend_with_edges(&[
			(a, b, -3.0), (a, d, 2.0),
			(b, a,  5.0), (b, c, 3.0),
			(c, a,  1.0),
			(d, a, -1.0), (d, c, 4.0),
		]); 

		// From specific source vertices to all the other (destination) vertices
		// From vertex 'a'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(a,0.),
			(b,0.),
			(c,0.),
			(d,1.),
		].iter().cloned().collect();
		let costs = johnson(&graph,a,None).unwrap();
		assert_eq!(costs,expected_costs);

		// From vertex 'b'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(a,1.),
			(b,0.),
			(c,0.),
			(d,2.),
		].iter().cloned().collect();
		let costs = johnson(&graph,b,None).unwrap();
		assert_eq!(costs,expected_costs);

		// From vertex 'c'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(a,1.),
			(b,1.),
			(c,0.),
			(d,2.),
		].iter().cloned().collect();
		let costs = johnson(&graph,c,None).unwrap();
		assert_eq!(costs,expected_costs);

		// From vertex 'd'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(a,0.),
			(b,0.),
			(c,0.),
			(d,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,d,None).unwrap();
		assert_eq!(costs,expected_costs);

		// Picking some specific routes
		// 'd' -> 'b'
		let expected_costs: HashMap<NodeIndex, f32> = [
			(b,0.),
		].iter().cloned().collect();
		let costs = johnson(&graph,d,Some(b)).unwrap();
		assert_eq!(costs,expected_costs);
	}

    #[test]
	fn test_case3() {
		// Same graph as in test #2, but with negative cycle, so it causes an error

		let mut graph = DiGraph::<&str,f32>::new();
		let a = graph.add_node("a");
		let b = graph.add_node("b");
		let c = graph.add_node("c");
		let d = graph.add_node("d");
		graph.extend_with_edges(&[
			(a, b, -3.0), (a, d, 2.0),
			(b, a,  5.0), (b, c, 3.0),
			(c, a,  1.0),
			(d, a, -1.0), (d, c, 4.0),
			(c, b, -5.0), 				// <-- negative cycle!
		]); 

		// Traverse through negative cycle
		// 'd' -> 'b'
		let costs = johnson(&graph,d,Some(b));
		assert!(costs.is_err());
	}
}
