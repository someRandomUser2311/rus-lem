#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($x);)+
        v
    }};
}

#[macro_export]
macro_rules! sum_all {
    () => {
        0
    };
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $first + $first $(+ $rest)*
    };
}
