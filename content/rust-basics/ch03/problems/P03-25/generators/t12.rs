fn main() {
    let mut g = Lcg(0x03000019);
    for _ in 0..300 {
        let cells: Vec<u32> = (0..9).map(|_| g.pick(256)).collect();
        let target = g.pick(256);
        call(format!(
            "find_cell([[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]], {target})",
            cells[0], cells[1], cells[2], cells[3], cells[4], cells[5], cells[6], cells[7], cells[8]
        ));
    }
}
