type Maze = Vec<Vec<bool>>;

fn resolve(maze: &Maze) -> i32 {
    let height = maze.len();
    let width = maze[0].len();
    let mut dp = vec![vec![0; width]; height];

    let base: i32 = 10;
    let d = base.pow(9) + 7;

    dp[0][0] = 1;
    for y in 0..height {
        for x in 0..width {
            if x == 0 && y == 0 {
                continue;
            }
            if !maze[y][x] {
                continue;
            }
            if x > 0 {
                dp[y][x] = dp[y][x] + dp[y][x - 1];
            }
            if y > 0 {
                dp[y][x] = dp[y][x] + dp[y - 1][x];
            }
            dp[y][x] = dp[y][x] % d;
        }
    }

    dp[height - 1][width - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let maze = vec![
            vec![true, true, true, false],
            vec![true, false, true, true],
            vec![true, true, true, true],
        ];
        assert_eq!(resolve(&maze), 3);
    }

    #[test]
    fn sample_02() {
        let maze = vec![
            vec![true, true],
            vec![false, true],
            vec![true, true],
            vec![true, false],
            vec![true, true],
        ];
        assert_eq!(resolve(&maze), 0);
    }

    #[test]
    fn sample_03() {
        let maze = vec![
            vec![true, true, false, true, true],
            vec![true, true, true, true, true],
            vec![false, true, true, true, false],
            vec![true, true, true, true, true],
            vec![true, true, false, true, true],
        ];
        assert_eq!(resolve(&maze), 24);
    }
}

fn main() {
    println!("Hello, world!");
}
