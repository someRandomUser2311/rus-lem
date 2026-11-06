use trpl::StreamExt;

pub async fn even_squares(values: Vec<u32>) -> Vec<u64> {
    let mut stream = trpl::stream_from_iter(values)
        .filter(|v| v % 2 == 1)
        .map(|v| v as u64 * v as u64);
    let mut out = Vec::new();
    while let Some(x) = stream.next().await {
        out.push(x);
    }
    out
}
