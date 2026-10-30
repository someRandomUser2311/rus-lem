pub fn find_cell(grid: [[u8; 3]; 3], target: u8) -> (i32, i32) {
    let mut row = 0;
    while row < 3 {
        let mut col = 0;
        while col < 3 {
            if grid[row][col] == target {
                return (row as i32, col as i32);
            }
            col += 1;
        }
        row += 1;
    }
    (-1, -1)
}
