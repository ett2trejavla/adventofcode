from queue import Queue




def read_input():
    edges={}
    with open("input.in") as f:
        for line in f.readlines():
            orbit =line.strip().split(')')
            if orbit[0] in edges.keys():
                edges[orbit[0]].append(orbit[1])
            else:
                edges[orbit[0]]=[orbit[1]]
        return edges

def find_orbits(edges):
    n_orb=0
    nodesqueue = Queue()
    nodesqueue.put(("COM",0))
    while not nodesqueue.empty():
        node,d = nodesqueue.get()
        n_orb+=d
        for orbit in edges[node]:
            if orbit in edges.keys():
                nodesqueue.put((orbit,d+1))
            else:
                n_orb+=d+1
    print(n_orb)
find_orbits(read_input())