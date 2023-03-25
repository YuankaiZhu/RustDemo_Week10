# Rust Demo Week10

In this demo, DFS and BFS algorithms are implemented using Rust. You can use it as following example:

```
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 1
Enter edge (format: A-B): a-b
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: w
Invalid choice. Try again.
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 1
Enter edge (format: A-B): c
Invalid edge format.
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 1
Enter edge (format: A-B): b-c
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 1
Enter edge (format: A-B): b-d
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 1
Enter edge (format: A-B): d-e
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 2
Graph: {Node('d'): [Node('b'), Node('e')], Node('e'): [Node('d')], Node('a'): [Node('b')], Node('c'): [Node('b')], Node('b'): [Node('a'), Node('c'), Node('d')]}
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 3
Enter starting node: e
DFS order: [Node('e'), Node('d'), Node('b'), Node('a'), Node('c')]
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 4
Enter starting node: c
BFS order: [Node('c'), Node('b'), Node('a'), Node('d'), Node('e')]
Menu:
1. Add edge
2. Show graph
3. Perform DFS
4. Perform BFS
5. Exit
Enter your choice: 5
```

