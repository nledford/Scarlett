use std::collections::HashMap;

fn divide(a: i32, b: i32) -> f64 {
    let fa = f64::from(a);
    let fb = f64::from(b);

    fa / fb
}

fn build_ratios() -> HashMap<String, f64> {
    let mut ratios = HashMap::new();

    // landscape
    ratios.insert("5:4".to_string(), divide(5, 4));
    ratios.insert("4:3".to_string(), divide(4, 3));
    ratios.insert("3:2".to_string(), divide(3, 2));
    ratios.insert("16:10".to_string(), divide(16, 10));
    ratios.insert("5:3".to_string(), divide(5, 3));
    ratios.insert("16:9".to_string(), divide(16, 9));
    ratios.insert("21:9".to_string(), divide(21, 9));
    ratios.insert("21:10".to_string(), divide(21, 10));

    // portrait
    ratios.insert("4:5".to_string(), divide(4, 5));
    ratios.insert("3:4".to_string(), divide(3, 4));
    ratios.insert("2:3".to_string(), divide(2, 3));
    ratios.insert("10:16".to_string(), divide(10, 16));
    ratios.insert("3:5".to_string(), divide(3, 5));
    ratios.insert("9:16".to_string(), divide(9, 16));
    ratios.insert("9:21".to_string(), divide(9, 21));
    ratios.insert("10:21".to_string(), divide(10, 21));

    // default
    ratios.insert("1:1".to_string(), divide(1, 1));

    ratios
}

fn find_closest_ratio(ratio: f64) -> String {
    let mut lowest_diff = 9999999999.0;
    let mut best_std = "1:1".to_string();

    for (key, value) in build_ratios() {
        let diff = (value - ratio).abs();

        if diff < lowest_diff {
            lowest_diff = diff;
            best_std = key;
        }
    }

    best_std.to_string()
}

pub fn extract_ratio(width: i32, height: i32) -> String {
    let divided = divide(width, height);
    if divided == 1.0 {
        return "1:1".to_string();
    }
    find_closest_ratio(divided)
}
