use std::collections::VecDeque;

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let d_move = vec![(-1_i32, 0_i32), (0, -1), (1, 0), (0, 1)];
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    let len_r = maze.len();
    let len_c = maze[0].len();
    let mut visited = vec![vec![false; len_c]; len_r];
    let mut parents = vec![vec![None::<(usize, usize)>; len_c]; len_r];
    let mut answer = Vec::<(usize, usize)>::new();

    visited[start.0][start.1] = true;

    deque.push_back(start);

    while !deque.is_empty() {
        let Some((r, c)) = deque.pop_front() else {
            break;
        };

        if (r, c) == end {
            break;
        }

        for (dr, dc) in d_move.iter() {
            let next_r = r as i32 + dr;
            let next_c = c as i32 + dc;
            if next_r < 0 || next_r >= len_r.try_into().unwrap() {
                continue;
            }
            if next_c < 0 || next_c >= len_c.try_into().unwrap() {
                continue;
            }

            let next_r: usize = next_r.try_into().unwrap();
            let next_c: usize = next_c.try_into().unwrap();
            if maze[next_r][next_c] != '#' && !visited[next_r][next_c] {
                visited[next_r][next_c] = true;
                parents[next_r][next_c] = Some((r, c));
                deque.push_back((next_r, next_c));
            }
        }
    }

    if !visited[end.0][end.1] {
        return vec![];
    }

    let mut path = end;
    loop {
        answer.push(path);

        if (path.0, path.1) == start {
            break;
        }
        path = parents[path.0][path.1].unwrap();
    }
    answer.reverse();
    answer
}
