import networkx as nx

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

def find(parent, i):
    if parent[i] == i:
        return i
    parent[i] = find(parent, parent[i])
    return parent[i]

def union(parent, rank, x, y):
    x_root = find(parent, x)
    y_root = find(parent, y)
    
    if rank[x_root] < rank[y_root]:
        parent[x_root] = y_root
    elif rank[x_root] > rank[y_root]:
        parent[y_root] = x_root
    else:
        parent[y_root] = x_root
        rank[x_root] += 1
        
def kruskal_algorithm(G):
    sorted_edges = sorted(G.edges(data=True), key=lambda x: x[2]['weight'])
    parent = {}
    rank = {}
    
    for node in G.nodes():
        parent[node] = node
        rank[node] = 0
    
    min_span_tree = []
    edge_selection_order = []
    
    for edge in sorted_edges:
        u, v, weight = edge
        u_root = find(parent, u)
        v_root = find(parent, v)
        
        if u_root != v_root:
            min_span_tree.append(edge)
            union(parent, rank, u_root, v_root)
            edge_selection_order.append("({}, {}) {}".format(u, v, weight['weight']))
        else:
            edge_selection_order.append("({}, {}) {} [cycle formed]".format(u, v, weight['weight']))
            
    return min_span_tree, edge_selection_order

min_span_tree, edge_selection_order = kruskal_algorithm(G)

print("Order in which edges are selected:")
for order in edge_selection_order:
    print(order)