pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // ·*·*·
    // ··*··
    // ··*··
    // ·····
    let mut result: Vec<String> = Vec::new();
    let mut grid = vec![];
    for i in minefield {
        grid.push(i.as_bytes());
    }
    let rows = minefield.len();
    let columns = if minefield.is_empty() {
        0
    } else {
        minefield[0].len()
    };
    for (y, row) in grid.iter().enumerate() {
        let mut row_result = String::new();
        for (x, _) in row.iter().enumerate() {
            // 如果我不是炸彈，我要去看9宮格的範圍，有沒有炸彈
            // 如果我是炸彈，那我沒有事要做，可以跳過
            let mut bomb_count = 0;
            if grid[y][x] == 32 {
                if y as isize - 1 >= 0 && x + 1 < columns {
                    // 右上
                    if grid[y - 1][x + 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if y as isize - 1 >= 0 && x as isize - 1 >= 0 {
                    // 左上
                    if grid[y - 1][x - 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if y + 1 < rows && x as isize - 1 >= 0 {
                    // 左下
                    if grid[y + 1][x - 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if y + 1 < rows && x + 1 < columns {
                    // 右下
                    if grid[y + 1][x + 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if y as isize - 1 >= 0 {
                    // 上方
                    if grid[y - 1][x] != 32 {
                        bomb_count += 1;
                    }
                }
                if x + 1 < columns {
                    // 右方
                    if grid[y][x + 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if x as isize - 1 >= 0 {
                    // 左方
                    if grid[y][x - 1] != 32 {
                        bomb_count += 1;
                    }
                }
                if y + 1 < rows {
                    // 下方
                    if grid[y + 1][x] != 32 {
                        bomb_count += 1;
                    }
                }
                if bomb_count == 0 {
                    row_result.push(' ');
                } else {
                    row_result.push_str(&bomb_count.to_string());
                }
            } else if grid[y][x] == 42 {
                row_result.push('*');
            } else {
                row_result.push(' ');
            }
        }
        result.push(row_result);
    }
    result
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
}
