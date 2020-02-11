use json;
use json::array;
use json::object;
pub fn json_string() -> std::string::String {
    let json_obj = object! {
        "score" => 44.0,
        "execution_time" => 136,
        "output" => "Text relevant to the entire submission",
        "stdout_visibility" => "visible",
        "extra_data" => object!{},
        "tests" => array![
            object!{
                "score"=> 2.0,
                "max_score"=> 2.0,
                "name"=> "Your name here",
                "number"=> 1.1,
                "output"=> "Giant multiline string that will be placed in a <pre> tag and collapsed by default",
                "tags"=> array!["tag1", "tag2", "tag3"],
                "visibility"=> "visible",
                "extra_data"=> object!{}
            }
        ],
    };
    return json::stringify(json_obj);
}
