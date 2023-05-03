import networkx as nx
import matplotlib.pyplot as plt

G = nx.Graph(['ab', 'ae', 'ag', 'bf', 'bg', 'cd', 'cg', 'ch', 'dh', 'fg'])

# Initialize the BFS tree and a queue with the starting vertex
bfs_tree = nx.DiGraph()
queue = ['a']

# Perform BFS traversal
while queue:
    current_node = queue.pop(0)
    neighbors = sorted(G.neighbors(current_node))
    
    for neighbor in neighbors:
        if neighbor not in bfs_tree:
            bfs_tree.add_edge(current_node, neighbor)
            queue.append(neighbor)

print("BFS tree starting at vertex 'a' with alphabetical order traversal:")
for edge in bfs_tree.edges():
    print(f"{edge[0]} -> {edge[1]}")

DG = nx.DiGraph(bfs_tree)

# Draw the directed graph with arrows
pos = nx.spring_layout(DG, seed=42)
nx.draw_networkx_edges(DG, pos, arrows=True)
nx.draw_networkx_labels(DG, pos)
plt.axis('off')
plt.show()
