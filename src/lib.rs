use std::{
	fmt::Debug,
	clone::Clone,
	ops::{Sub,Add},
};
use std::collections::HashMap;
use petgraph::{
	prelude::*,
	{EdgeType},
	graph::{NodeIndex,IndexType,Graph},
	algo::{dijkstra, bellman_ford,FloatMeasure,Measure,NegativeCycle},
};

mod tests;
/// Johnson's algorithm. Reuses Dijksra and Bellman Ford
pub fn johnson<N,E,Ty,Ix>(orig_graph: &Graph<N,E,Ty,Ix>, start: NodeIndex<Ix>, goal: Option<NodeIndex<Ix>>) 
	-> Result<HashMap<NodeIndex<Ix>, E>,NegativeCycle>
	where 
		N: Default + Debug,
		E: FloatMeasure + Debug + Sub<Output=E> + Add<Output=E> + Default + Measure,
		Ty: EdgeType,
        Ix: IndexType,
		Graph<N, E, Ty, Ix> : Clone,
{
	// Preparation. Cloning the graph is a requirement as all the edges will have to be altered
	let mut graph = orig_graph.clone();

	// Step 1. Add 'Helper' vertex to each Vertex with weight 0
	let indices: Vec<NodeIndex<Ix>> = graph.node_indices().into_iter().collect();
	let helper_vertex = graph.add_node(N::default());
	let mut helper_edges: Vec<EdgeIndex<Ix>> = Vec::new();
	for destination in &indices {
		// Note: implemenation caveat: default() must produce 0s here
		let eidx = graph.add_edge(helper_vertex,*destination,E::default());		
		helper_edges.push(eidx);
	}

	// Step 2. Run Bellman Ford algo to calculate shortest path from helper vertex to the other vertices
	let result = bellman_ford(&graph, helper_vertex);
	let (path_costs,_predecessors) = match result {
		Ok((costs,nodes)) => {
				let mut i: usize =0;
				let mut m: HashMap<NodeIndex<Ix>,E> = HashMap::new();
				for n in &nodes {
					match n {
						Some(nidx) => {
							m.insert(*nidx,costs[i]);
						},
						None => (),
					}
					i += 1;
				}
				(costs,nodes)
		},
		Err(neg_cycle) => {
			return Err(neg_cycle);
		},
	};

	// Step 3. Re-weigth the graph to eliminate negative weight (using previous calculations)
	let edge_indices: Vec<EdgeIndex<Ix>> = graph.edge_indices().collect();
	let mut previous_weights: Vec<E> = Vec::new();
	for eidx in &edge_indices {
		previous_weights.push(*graph.edge_weight(*eidx).unwrap());
	}
	for eidx in edge_indices {
		let (src_vertex,dst_vertex) = graph.edge_endpoints(eidx).unwrap();
		let previous_weight = *graph.edge_weight(eidx).unwrap();
		let src_vertex_cost = path_costs[src_vertex.index()];
		let dst_vertex_cost = path_costs[dst_vertex.index()];
		let new_weight  = previous_weight + src_vertex_cost - dst_vertex_cost ;
		let weight_ref = graph.edge_weight_mut(eidx).unwrap();
		*weight_ref = new_weight;
	}

	// Step 4. Remove auxiliary Vertices/Edges
	for eidx in &helper_edges {
		graph.remove_edge(*eidx);
	}
	graph.remove_node(helper_vertex);

	// Step 5. Run Dijkstra on adjusted graph and return the result
	let shortest_paths= dijkstra(&graph,start,goal, |edge|*edge.weight());
	match goal {
		Some(vidx) => {
			// only package the Vertex that has been provided as 'goal' vertex (drop the rest)
			let mut output: HashMap<NodeIndex<Ix>,E> = HashMap::new();
			match shortest_paths.get(&vidx) {
				Some(cost) => {let _ = output.insert(vidx,*cost); ()},
				None => (),
			}
			Ok(output)
		},
		None => Ok(shortest_paths)
	}
}
