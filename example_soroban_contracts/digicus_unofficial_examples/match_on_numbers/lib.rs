fn match_on_numbers(x: String) -> String {
    match x.as_str() {
        "zero" => "zero",
        "one" => "one",
        _ => "unknown",
    }
}
