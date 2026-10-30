pub fn find_cell(grid: [[u8; 3]; 3], target: u8) -> (i32, i32) {
    let mut row = 0;
    let mut found = (-1, -1);
    'search: loop {
        if row == 3 {
            break 'search found;
        }
        let mut col = 0;
        for value in grid[row] {
            if value == target {
                found = (row as i32, col);
            }
            col += 1;
        }
        row += 1;
    }
}
