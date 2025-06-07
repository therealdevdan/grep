use minigrep::search;
use minigrep::search_case_insensitive;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &'static str = "duct";
        let contents: &'static str = "\
Rust:
safe, fast, productive.
Pick there,";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &'static str = "rUst";
        let contents: &'static str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
} 
