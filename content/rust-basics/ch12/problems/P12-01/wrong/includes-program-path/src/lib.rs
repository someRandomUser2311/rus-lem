pub fn echo_args(args: &[String]) -> String {
    let mut report = format!("Arguments: {}\n", args.len());
    let mut position = 0;
    for arg in args {
        position += 1;
        report.push_str(&format!("{position}: {arg}\n"));
    }
    report
}
