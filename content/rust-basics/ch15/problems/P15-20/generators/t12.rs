// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000014);
    for _ in 0..300 {
        let n = g.pick(40);
        let v: Vec<i32> = (0..n).map(|_| g.pick(200) as i32 - 100).collect();
        call(format!("{{ let t = crate::balanced(&{v:?}); (crate::inorder(&t), crate::height(&t)) }}"));
    }
}
