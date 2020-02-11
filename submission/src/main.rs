mod lib;

const JSON_STRING: &str = r#"{
  "score": 44.0,
  "execution_time": 136,
  "output": "Text relevant to the entire submission",
  "visibility": "after_due_date",
  "stdout_visibility": "visible",
  "extra_data": {},
  "tests": [
        {
            "score": 2.0,
            "max_score": 2.0,
            "name": "Your name here",
            "number": "1.1",
            "output": "Giant multiline string that will be placed in a <pre> tag and collapsed by default",
            "tags": ["tag1", "tag2", "tag3"],
            "visibility": "visible",
            "extra_data": {}
        }
    ],
  "leaderboard":
    [
      {"name": "Accuracy", "value": 0.926},
      {"name": "Time", "value": 15.1, "order": "asc"},
      {"name": "Stars", "value": "*****"}
    ]
}"#;

fn main() -> std::io::Result<()> {
    return lib::write(JSON_STRING, "/tmp/results.json");
}
