pub fn collect_results(values: Vec<Result<u32, String>>) -> Result<Vec<u32>, String> {
    Ok(values.into_iter().filter_map(|r| r.ok()).collect())
}
