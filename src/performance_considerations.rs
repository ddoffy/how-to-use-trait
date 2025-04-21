pub trait Parser {
    type Output;
    type Error;

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error>;
    fn name(&self) -> &'static str;
}

struct JsonParser;
struct XmlParser;
struct YamlParser;

impl Parser for JsonParser {
    type Output = String;
    type Error = String;

    fn parse(&self, _input: &str) -> Result<Self::Output, Self::Error> {
        println!("Parsing JSON data");
        Ok("Parsed JSON".to_string())
    }

    fn name(&self) -> &'static str {
        "JSON"
    }
}

impl Parser for XmlParser {
    type Output = String;
    type Error = String;

    fn parse(&self, _input: &str) -> Result<Self::Output, Self::Error> {
        println!("Parsing XML data");
        Ok("Parsed XML".to_string())
    }

    fn name(&self) -> &'static str {
        "XML"
    }
}

impl Parser for YamlParser {
    type Output = String;
    type Error = String;

    fn parse(&self, _input: &str) -> Result<Self::Output, Self::Error> {
        println!("Parsing YAML data");
        Ok("Parsed YAML".to_string())
    }

    fn name(&self) -> &'static str {
        "YAML"
    }
}

fn parse_config<P: Parser>(parser: P, input: &str) -> Result<P::Output, P::Error> {
    let start_time = std::time::Instant::now();
    let result = parser.parse(input);
    let duration = start_time.elapsed();

    println!("Parse with {} in {:?}", parser.name(), duration);

    // Convert result to string (simplified)
    result
}

// Dynamic dispatch version (small overhead, more flexible)
fn parse_any_format(input: &str, format: &str) -> Result<String, String> {
    let parse: Box<dyn Parser<Output = String, Error = String>> = match format {
        "JSON" => Box::new(JsonParser),
        "XML" => Box::new(XmlParser),
        "YAML" => Box::new(YamlParser),
        _ => return Err("Unknown parser".to_string()),
    };

    let start_time = std::time::Instant::now();
    let result = parse.parse(input);
    let duration = start_time.elapsed();

    println!("Parse with {} in {:?}", parse.name(), duration);

    // Convert result to string (simplified)
    result
}

pub fn demonstrate_performance_considerations() {
    let json_parser = JsonParser;
    let xml_parser = XmlParser;
    let yaml_parser = YamlParser;

    let input = r#"{"key": "value"}"#;

    // Static dispatch
    let _ = parse_config(json_parser, input.clone());
    let _ = parse_config(xml_parser, input.clone());
    let _ = parse_config(yaml_parser, input.clone());

    // Dynamic dispatch
    let json_parser = "JsonParser";
    let xml_parser = "XmlParser";
    let yaml_parser = "YamlParser";

    let _ = parse_any_format(input, json_parser);
    let _ = parse_any_format(input, xml_parser);
    let _ = parse_any_format(input, yaml_parser);
}
