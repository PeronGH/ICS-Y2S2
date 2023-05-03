import numpy as np
import networkx as nx

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

# Sort edges in alphabetical order
edges = sorted(G.edges())
edges = [(min(x, y), max(x, y)) for x, y in edges]

# Assign each edge with a number
edge_numbers = {edge: i for i, edge in enumerate(edges)}

# Convert NodeView to a list
nodes_list = list(sorted(G.nodes()))

# Create incidence matrix
incidence_matrix = np.zeros((len(G.edges()), len(G.nodes())), dtype=int)
for edge, edge_num in edge_numbers.items():
    incidence_matrix[edge_num, nodes_list.index(edge[0])] = 1
    incidence_matrix[edge_num, nodes_list.index(edge[1])] = 1

# Create incidence list
incidence_list = {i: [edge[0], edge[1]] for i, edge in enumerate(edges)}

# Print sorted edges
print("Sorted Edges:")
for edge in edges:
    print(f"  {edge}")

# Print edge numbers
print("\nEdge Numbers:")
for edge, number in edge_numbers.items():
    print(f"  e{number}: {edge}")

# Print incidence matrix
print("\nIncidence Matrix:")
print(" " * 4, end="")
for node in nodes_list:
    print(f" {node}", end=" ")
print()
for i, row in enumerate(incidence_matrix):
    print(f"e{i}: ", end="")
    for val in row:
        print(f" {val}", end=" ")
    print()

# Print incidence list
print("\nIncidence List:")
for edge_num, vertices in incidence_list.items():
    print(f"  e{edge_num}: {vertices}")


