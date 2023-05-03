import networkx as nx
from networkx.algorithms import tree

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


# Write a function to find the shortest paths from the vertex a to all other vertices in the graph G using Dijkstra’s algorithm.
# Show the distance from the selected tree to **each** node step by step and give the order in which edges are selected.
from prettytable import PrettyTable

def dijkstra_shortest_paths(G, source):
    visited = {source: 0}
    path = {}
    selected_nodes = set()

    nodes = list(sorted(G.nodes))

    # Create a PrettyTable object
    table = PrettyTable()
    table.field_names = ["Order Selected"] + [f"{node}(-, ∞)" if node != source else f"{node}(0, -)" for node in G.nodes]

    while nodes:
        min_node = None
        for node in nodes:
            if node in visited:
                if min_node is None:
                    min_node = node
                elif visited[node] < visited[min_node]:
                    min_node = node

        if min_node is None:
            break

        nodes.remove(min_node)
        current_weight = visited[min_node]
        selected_nodes.add(min_node)

        for edge in G.edges(min_node, data=True):
            weight = current_weight + edge[2]['weight']
            neighbor = edge[1]
            if neighbor not in visited or weight < visited[neighbor]:
                visited[neighbor] = weight
                path[neighbor] = min_node

        selected_edge = f"{min_node}({path[min_node]}, {G[path[min_node]][min_node]['weight']})" if min_node != source else f"{min_node}(0, -)"

        # Add a row to the table
        row_data = [selected_edge]
        for node in G.nodes:
            if node not in selected_nodes:
                connected_node = path.get(node)
                if node in visited:
                    if connected_node:
                        row_data.append(f"{node}({connected_node}, {visited[node]})")
                    else:
                        row_data.append(f"{node}(-, {visited[node]})")
                else:
                    row_data.append(f"{node}(-, ∞)")
            else:
                row_data.append("")
        table.add_row(row_data)

    return path, table

shortest_paths, table = dijkstra_shortest_paths(G, 'a')
print(table)
print("\nOrder of selected edges with weights:")
for node, prev_node in shortest_paths.items():
    weight = G[prev_node][node]['weight']
    print(f"{prev_node} -{weight}- {node}")
