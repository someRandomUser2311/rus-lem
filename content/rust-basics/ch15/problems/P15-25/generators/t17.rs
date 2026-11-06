// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000019);
    for _ in 0..300 {
        let s1 = g.pick(32); let x = g.next() >> s1;
        let s2 = g.pick(32); let y = g.next() >> s2;
        let s3 = g.pick(32); let amt = g.next() >> s3;
        let mk = "std::rc::Rc::new(std::cell::RefCell::new";
        let text = match g.pick(4) {
            0 => format!("{{ let a = {mk}({x}u32)); let r = crate::transfer(&a, &a, {amt}); (r, *a.borrow(), *a.borrow()) }}"),
            1 => format!("{{ let a = {mk}({x}u32)); let b = {mk}({y}u32)); let guard = b.borrow(); let r = crate::transfer(&a, &b, {amt}); drop(guard); (r, *a.borrow(), *b.borrow()) }}"),
            _ => format!("{{ let a = {mk}({x}u32)); let b = {mk}({y}u32)); let r = crate::transfer(&a, &b, {amt}); (r, *a.borrow(), *b.borrow()) }}"),
        };
        call(text);
    }
}
