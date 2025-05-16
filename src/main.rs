mod lexer;

fn main() {
    use regex::Regex;

    let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
    let hay = "\
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
";

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
        match lineno.parse::<u64>() {
            Ok(lineno) => {
                results.push((path, lineno, line))
            },
            Err(_) => {
                //
            }
        }
    }
    assert_eq!(results, vec![
        ("path/to/foo", 54, "Blue Harvest"),
        ("path/to/bar", 90, "Something, Something, Something, Dark Side"),
        ("path/to/baz", 3, "It's a Trap!"),
    ]);
    println!("Hello, world!");
}
