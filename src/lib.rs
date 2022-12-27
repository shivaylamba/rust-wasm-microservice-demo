use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
struct Record {
    a: i64,
    b: f64,
    c: String,
}

fn json_to_dataframe(json: &str) -> Vec<Record> {
    let records: Vec<Record> = from_str(json).unwrap();
    records
}

fn main() {
    let json = r#"[
        {"a": 1, "b": 2.5, "c": "foo"},
        {"a": 2, "b": 3.5, "c": "bar"},
        {"a": 3, "b": 4.5, "c": "baz"}
    ]"#;

    let df = json_to_dataframe(json);
    for record in df {
        println!("{:?}", record);
    }
}





