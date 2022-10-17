use serde_json::Value;

fn main() {
    let input = std::fs::read_to_string("input/2015/12.json").expect("input should exist");

    let obj = match serde_json::from_str(&input) {
        Ok(j) => j,
        Err(_) => Value::Null,
    };

    let sum = json_sum(&obj, false);
    let sum_without_red = json_sum(&obj, true);

    println!("The sum of all numbers in the JSON document is {} and {} when excluding objects with the value \"red\".", sum, sum_without_red)
}

fn json_sum(input: &Value, exclude_red: bool) -> i64 {
    match input {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => {
            let mut sum = 0;
            for val in arr {
                sum += json_sum(&val, exclude_red);
            }
            sum
        }
        Value::Object(obj) => {
            let mut sum = 0;
            for (_, val) in obj {
                if exclude_red && val == "red" {
                    return 0;
                }

                sum += json_sum(&val, exclude_red);
            }
            sum
        }
    }
}
