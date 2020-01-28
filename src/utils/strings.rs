pub fn contains_sort_order(item: &str) -> bool {
    let first_char = item.chars().next().unwrap();

    match first_char {
        '+' | '-' => true,
        _ => false,
    }
}

pub fn get_category_from_sort(item: &str) -> &str {
    if contains_sort_order(item) {
        item.chars()
            .next()
            .map(|c| &item[c.len_utf8()..])
            .unwrap()
    } else {
        item
    }
}
