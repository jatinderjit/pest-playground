use std::error::Error;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
struct CSVParser;

pub fn parse(data: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = CSVParser::parse(Rule::file, &data)?
        // extracts the "file" rule
        .next()
        // `unwrap` should never fail
        .unwrap();

    let rows = file
        .into_inner()
        .filter_map(|record| match record.as_rule() {
            Rule::record => {
                let row = record
                    .into_inner()
                    .map(|number| number.as_str().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                Some(row)
            }
            Rule::EOI => None,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    Ok(rows)
}

#[cfg(test)]
mod test {

    use super::{parse, CSVParser, Rule};
    use pest::Parser;

    #[test]
    fn test_numbers() {
        let parse = CSVParser::parse(Rule::number, "0");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "0");

        let parse = CSVParser::parse(Rule::number, "10");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "10");

        let parse = CSVParser::parse(Rule::number, "-0");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "-0");

        let parse = CSVParser::parse(Rule::number, "-10");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "-10");

        let parse = CSVParser::parse(Rule::number, "273.15");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "273.15");

        let parse = CSVParser::parse(Rule::number, "-273.15");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "-273.15");

        let parse = CSVParser::parse(Rule::number, "004");
        assert!(parse.is_err());

        let parse = CSVParser::parse(Rule::number, "not a number");
        assert!(parse.is_err());

        let parse = CSVParser::parse(Rule::number, ".4");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), ".4");

        let parse = CSVParser::parse(Rule::number, "-.4");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "-.4");

        let parse = CSVParser::parse(Rule::number, "1.1.1");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "1.1");

        let parse = CSVParser::parse(Rule::number, "1a");
        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().as_str(), "1");

        let parse = CSVParser::parse(Rule::number, "a1");
        assert!(parse.is_err());
    }

    #[test]
    fn test_parse() {
        let unparsed = r"0,1
-2,-3.4";
        let parsed = parse(unparsed);
        let data = parsed.unwrap();
        assert_eq!(data, vec![vec![0., 1.], vec![-2., -3.4]]);
    }
}
