import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph(['ab', 'ae', 'ag', 'bf', 'bg', 'cd', 'cg', 'ch', 'dh', 'fg'])

# Custom DFS function to ensure vertex alphabetical order traversal
def dfs_alpha_order(graph, start):
    visited = set()
    stack = [(start, None)]

    while stack:
        node, parent = stack.pop()
        if node not in visited:
            visited.add(node)
            if parent is not None:
                yield parent, node

            neighbors = sorted(n for n in graph.neighbors(node) if n not in visited)
            stack.extend((neighbor, node) for neighbor in reversed(neighbors))

# Perform DFS traversal starting from vertex 'a'
dfs_edges = list(dfs_alpha_order(G, 'a'))

for edge in dfs_edges:
    print(f"{edge[0]} -> {edge[1]}")

# Create a directed graph from the DFS traversal edges
DG = nx.DiGraph(dfs_edges)

# Draw the directed graph with arrows
pos = nx.spring_layout(DG, seed=42)
nx.draw_networkx_edges(DG, pos, arrows=True)
nx.draw_networkx_labels(DG, pos)
plt.axis('off')
plt.show()