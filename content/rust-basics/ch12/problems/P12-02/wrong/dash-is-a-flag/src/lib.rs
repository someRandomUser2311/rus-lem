pub fn split_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut flags = Vec::new();
    let mut positionals = Vec::new();
    if args.is_empty() {
        return (flags, positionals);
    }
    let mut after = false;
    for arg in &args[1..] {
        if !after && arg == "--" {
            after = true;
        } else if !after && arg.starts_with('-') {
            flags.push(arg.clone());
        } else {
            positionals.push(arg.clone());
        }
    }
    (flags, positionals)
}
