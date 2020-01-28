/// Does exactly what it says on the tin.
///
/// # Example
///
/// ```
/// use scarlett_server::utils::strings::get_first_char_of_str;
///
/// let test_str = "Test string";
/// let first_char = get_first_char_of_str(test_str);
/// assert_eq!(first_char.unwrap(), 'T')
/// ```
pub fn get_first_char_of_str(string: &str) -> Option<char> {
    string.chars().next()
}

pub fn contains_sort_order(item: &str) -> bool {
    let first_char = get_first_char_of_str(item);

    if first_char.is_none() {
        return false
    }

    match first_char.unwrap() {
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
