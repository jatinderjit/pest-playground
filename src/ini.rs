use std::collections::HashMap;
use std::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ini.pest"]
struct INIParser;

pub struct Config {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub name: String,
    pub values: HashMap<String, String>,
}

pub fn parse(body: &str) -> Result<Config, Box<dyn Error>> {
    let file = INIParser::parse(Rule::file, body)?.next().unwrap();

    let mut sections = Vec::new();
    let mut section = Section { name: String::new(), values: HashMap::new()};
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                sections.push(section);
                let name = line.into_inner().next().unwrap().as_str().to_owned();
                section = Section { name, values: HashMap::new() };
            }
            Rule::property => {
                let mut property = line.into_inner();
                let name = property.next().unwrap().as_str().to_owned();
                let value = property.next().unwrap().as_str().to_owned();
                section.values.insert(name, value);
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    sections.push(section);
    Ok(Config { sections })
}

#[cfg(test)]
mod test {
    use pest::Parser;
    use super::{INIParser, Rule, parse};

    #[test]
    fn test_property() {
        let parse = INIParser::parse(Rule::property, "some_key=some_value");
        assert!(parse.is_ok());

        let property = parse.unwrap().next().unwrap();
        let mut pairs = property.into_inner();
        let key = pairs.next().unwrap().as_str();
        assert_eq!(key, "some_key");
        let value = pairs.next().unwrap().as_str();
        assert_eq!(value, "some_value");
    }

    #[test]
    fn test_parse() {
        let file = "
username=abc
password=pass

[server_1]
interface=eth0
ip=127.0.0.1
document_root=/var/www/example.org

[empty_section]

[second_server]
document_root=/var/www/example.com
ip=
interface=eth1
";
        let config = parse(file).unwrap();
        assert_eq!(config.sections.len(), 4);

        let mut sections = config.sections.iter();

        // Default section
        let section = sections.next().unwrap();
        assert!(section.name.is_empty());
        assert_eq!(section.values.len(), 2);
        assert_eq!(section.values.get(&"username".to_string()), Some(&"abc".to_string()));
        assert_eq!(section.values.get(&"password".to_string()), Some(&"pass".to_string()));

        // server_1
        let section = sections.next().unwrap();
        assert_eq!(&section.name, "server_1");
        assert_eq!(section.values.len(), 3);
        assert_eq!(section.values.get(&"interface".to_string()), Some(&"eth0".to_string()));
        assert_eq!(section.values.get(&"ip".to_string()), Some(&"127.0.0.1".to_string()));
        assert_eq!(section.values.get(&"document_root".to_string()), Some(&"/var/www/example.org".to_string()));

        // empty_section
        let section = sections.next().unwrap();
        assert_eq!(&section.name, "empty_section");
        assert!(section.values.is_empty());

        // second_server
        let section = sections.next().unwrap();
        assert_eq!(&section.name, "second_server");
        assert_eq!(section.values.len(), 3);
        assert_eq!(section.values.get(&"interface".to_string()), Some(&"eth1".to_string()));
        assert_eq!(section.values.get(&"ip".to_string()), Some(&"".to_string()));
        assert_eq!(section.values.get(&"document_root".to_string()), Some(&"/var/www/example.com".to_string()));
    }
}
