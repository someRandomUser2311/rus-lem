pub fn find_cell(grid: [[u8; 3]; 3], target: u8) -> (i32, i32) {
    let mut row = 0;
    'search: loop {
        if row == 3 {
            break 'search (-1, -1);
        }
        let mut col = 0;
        for value in grid[row] {
            if value == target {
                break 'search (col, row as i32);
            }
            col += 1;
        }
        row += 1;
    }
}
