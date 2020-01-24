pub fn contains_sort_order(item: &String) -> bool {
    let first_char = item.chars().next().unwrap();

    match first_char {
    '+' | '-' => true,
    _ => false,
    }
}

pub fn get_category_from_sort(item: &String) -> String {
    if contains_sort_order(item) {
        item.chars().next().map(|c| &item[c.len_utf8()..]).unwrap().to_string()
    } else {
        item.to_string()
    }
}