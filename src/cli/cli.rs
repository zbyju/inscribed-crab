pub fn parse_args(args: Vec<String>) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid number of arguments".to_string());
    }

    let file_path = args.get(1).unwrap();

    Ok(file_path.to_string())
}
