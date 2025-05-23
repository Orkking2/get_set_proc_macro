use get_set_macro::get_set;

#[get_set(get(inline_always, rename = "IGNORED_RENAME"))]
struct Example {
    #[gsflags(get(rename = "renamed_get_name"))]
    name: String,

    // Has no inline set, will override the global inline
    #[gsflags(get)]
    age: u32,

    // Still recieves the global flags
    unflagged: i64,

    // Will not have any getters or setters generated
    #[gsflags(skip)]
    skipped: f32,

    #[gsflags(get(inline_always, rename = "city_ref"), set(rename = "set_city" /* same as default */))]
    city: String,
}

fn main() {
    let mut example = Example {
        name: "ExampleName".to_string(),
        age: 55,
        unflagged: -128,
        skipped: 12.32,
        city: "ExampleCity".to_string(),
    };

    assert_eq!("ExampleName", example.get_name().as_str());
    assert_eq!(example.renamed_get_name(), example.get_name());
    assert_eq!(55, *example.get_age());
    assert_eq!("ExampleCity", example.city_ref().as_str());

    example.set_city("NewCity".to_string());

    assert_eq!("NewCity", example.city_ref().as_str());

    assert_eq!(-128, *example.get_unflagged());

    // The following would produce an error
    // example.get_skipped();
}