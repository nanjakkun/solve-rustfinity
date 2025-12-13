/*
Your Task
    Your task is to write a function solve_maze that takes a 2D vector of characters representing the maze and two tuples representing the start and end points. The maze will be given as a grid of characters where 'S' represents the start, 'E' represents the end, '.' represents an open path, and '#' represents a wall.
    The function should return a vector of tuples representing the path from start to end if a path exists, or an empty vector if no path exists.

Constraints
    The maze will always contain exactly one 'S' and one 'E'.
    You can only move up, down, left, or right.
    Implement the function using appropriate control flow constructs (loops, conditionals, and/or recursion).
    Ensure your solution handles edge cases, such as no available path or mazes with various sizes.
    If the maze is a dead-end, return an empty vector.
    If the maze has multiple solutions, return the shortest path.

Hints
    Collections:
        VecDeque: A double-ended queue from the std::collections module, which is useful for implementing a queue for BFS. Methods like push_back and pop_front will be helpful.

    Indexing:
        Use usize for indices and be cautious with arithmetic operations to avoid overflow. The wrapping_add method can help with safe arithmetic.

    2D Vector Initialization:
        Initialize 2D vectors for visited and path tracking. Use nested vec! macros for creating the initial structure.

    Backtracking Path:
        Once the end point is reached, backtrack using the path vector to reconstruct the path from end to start. Collect these coordinates in a vector and reverse it to get the path from start to end.

    Boundary Checks:
        Ensure the new coordinates are within the maze boundaries and check if the cell is a wall ('#') or already visited.
*/

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let rows = maze.len();
    if rows == 0 {
        return vec![];
    }
    let cols = maze[0].len();
    if cols == 0 {
        return vec![];
    }

    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![vec![false; cols]; rows];
    let mut parents = vec![vec![None; cols]; rows];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    // Up, Down, Left, Right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut found = false;

    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == end {
            found = true;
            break;
        }

        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] && maze[nr][nc] != '#' {
                    visited[nr][nc] = true;
                    parents[nr][nc] = Some((r, c));
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    if !found {
        return vec![];
    }

    let mut path = Vec::new();
    let mut curr = end;
    path.push(curr);

    while curr != start {
        if let Some(parent) = parents[curr.0][curr.1] {
            curr = parent;
            path.push(curr);
        } else {
            return vec![];
        }
    }

    path.reverse();
    path
}
