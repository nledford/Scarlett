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

/// Checks if a provided string contains a sort direction,
/// with + being ascending and - being descending
///
/// If no direction has been provided, the sort direction should be ascending by default
///
/// # Example
///
/// ```
/// use scarlett_server::utils::strings::contains_sort_order;
///
/// let category = "+file_name";
/// let has_sort_order = contains_sort_order(category);
/// assert_eq!(has_sort_order, true)
/// ```
pub fn contains_sort_order(item: &str) -> bool {
    let first_char = get_first_char_of_str(item);

    if first_char.is_none() {
        return false;
    }

    match first_char.unwrap() {
        '+' | '-' => true,
        _ => false,
    }
}

/// Removes the sort direction from the string and returns the category. (E.g., "+file_name" -> "file_name")
///
/// # Example
///
/// ```
/// use scarlett_server::utils::strings::get_category_from_sort;
///
/// let test_sort_str = "+file_name";
/// let category = get_category_from_sort(test_sort_str);
/// assert_eq!(category, "file_name")
/// ```
pub fn get_category_from_sort(item: &str) -> &str {
    if contains_sort_order(item) {
        item.chars().next().map(|c| &item[c.len_utf8()..]).unwrap()
    } else {
        item
    }
}
