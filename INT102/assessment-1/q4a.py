import networkx as nx
import numpy as np

G = nx.Graph([
    ('a', 'b'),
    ('a', 'd'),
    ('b', 'c'),
    ('b', 'e'),
    ('c', 'e'),
    ('c', 'f'),
    ('d', 'e'),
    ('e', 'f')
])

# Sort nodes and edges
sorted_nodes = sorted(G.nodes)

# (a) Adjacency matrix and adjacency list
adj_matrix = nx.adjacency_matrix(G, nodelist=sorted_nodes).todense()
adj_list = {node: sorted(list(G.neighbors(node))) for node in sorted_nodes}


# Print adjacency matrix and adjacency list
print("Adjacency Matrix:")
print(np.array(adj_matrix))
print("\nAdjacency List:")
for node, neighbors in adj_list.items():
    print(f"{node}: {neighbors}")
