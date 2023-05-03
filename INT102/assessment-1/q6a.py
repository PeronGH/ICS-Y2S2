import networkx as nx
from prettytable import PrettyTable

# Define the graph
G = nx.Graph()
G.add_edge('a', 'b', weight=4)
G.add_edge('a', 'e', weight=1)
G.add_edge('b', 'c', weight=6)
G.add_edge('b', 'f', weight=3)
G.add_edge('c', 'd', weight=7)
G.add_edge('c', 'f', weight=10)
G.add_edge('c', 'g', weight=12)
G.add_edge('d', 'g', weight=14)
G.add_edge('d', 'h', weight=8)
G.add_edge('e', 'f', weight=2)
G.add_edge('f', 'g', weight=5)
G.add_edge('g', 'h', weight=8)


def prims_mst(G, start_vertex=None):
    if start_vertex is None:
        start_vertex = list(G.nodes)[0]

    # Initialize the priority queue with infinite distances
    pq = {node: (float('inf'), None) for node in G.nodes}
    pq[start_vertex] = (0, None)

    # Initialize the table to record the distances at each step
    table = PrettyTable()
    table.field_names = ["Step", "Selected Order"] + list(G.nodes)

    visited = set()
    mst_edges = []

    while pq:
        # Find the minimum distance vertex in the priority queue
        current_vertex, (min_distance, min_prev_vertex) = min(
            pq.items(), key=lambda x: x[1])

        # Add the vertex to the visited set and remove it from the priority queue
        visited.add(current_vertex)
        del pq[current_vertex]

        # If the current vertex has a previous vertex, add the edge to the MST edges
        if min_prev_vertex is not None:
            mst_edges.append((min_prev_vertex, current_vertex))

        # Iterate through the neighbors of the current vertex
        for neighbor, edge_attrs in G[current_vertex].items():
            if neighbor not in visited:
                edge_weight = edge_attrs["weight"]
                if edge_weight < pq[neighbor][0]:
                    pq[neighbor] = (edge_weight, current_vertex)

        # Update the table with the current step information
        selected_order = f"{current_vertex}({min_prev_vertex if min_prev_vertex else 0}, {min_distance})"
        distances = []
        for node in G.nodes:
            dist, nearest_vertex = pq.get(node, (0, None))
            if nearest_vertex is not None:
                distances.append(f"{node}({nearest_vertex}, {dist})")
            else:
                distances.append("")

        row_data = [len(visited), selected_order] + distances
        table.add_row(row_data)

    print(table)
    return mst_edges


# continue to prove if the MST is unique (try different start vertices)
mst_results = []

# Try different starting vertices
for start_vertex in G.nodes:
    mst_edges = prims_mst(G, start_vertex)
    mst_results.append(mst_edges)

# Check if the MST is unique
is_unique = True
for i in range(1, len(mst_results)):
    if set(mst_results[i]) != set(mst_results[0]):
        is_unique = False
        break

if is_unique:
    print("The MST is unique.")
else:
    print("The MST is not unique.")
