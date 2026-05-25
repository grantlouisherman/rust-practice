use std::env;


pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut found_lines = vec![];
    let ignore_case =. env::var("IGNORE_CASE").is_ok();
    
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            found_lines.push(line);
        }
    }
    found_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, content));
    }

}