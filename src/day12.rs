use json::JsonValue;

fn part1(input: &str) -> i32 {
    input
        .split(|c| !matches!(c, '0'..='9' | '-'))
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .sum()
}

fn part2(input: &str) -> i32 {
    let json = json::parse(input).unwrap();
    fn handle_json(json: JsonValue) -> i32 {
        match json {
            JsonValue::Null => 0,
            JsonValue::Short(_) => 0,
            JsonValue::String(_) => 0,
            JsonValue::Number(number) => number.as_fixed_point_i64(0).unwrap() as _,
            JsonValue::Boolean(_) => 0,
            JsonValue::Object(mut object) => {
                for (_, v) in object.iter() {
                    if *v == JsonValue::String("red".into()) {
                        return 0;
                    }
                }

                let mut sum = 0;
                for (_, v) in object.iter_mut() {
                    sum += handle_json(std::mem::replace(v, JsonValue::Null));
                }
                sum
            }
            JsonValue::Array(json_values) => {
                let mut sum = 0;
                for v in json_values {
                    sum += handle_json(v);
                }
                sum
            }
        }
    }
    handle_json(json)
}

fn main() {
    let path = if cfg!(debug_assertions) {
        concat!("./input/", module_path!(), "-ex.txt")
    } else {
        concat!("./input/", module_path!(), ".txt")
    };
    let file = std::fs::read_to_string(path).unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 156366);
    assert_eq!(part2(file), 96852);
}
