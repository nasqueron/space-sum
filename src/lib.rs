const KB: f64 = 1024.0;
const MB: f64 = 1048576.0;
const GB: f64 = 1073741824.0;
const TB: f64 = 1099511627776.0;

/// Parses a size expression
///
/// # Arguments
///
/// * `expression`:
///
/// returns:
///   - None if the expression can't be parsed
///   - Some(size in bytes) if it can be parsed
///
/// # Examples
///
/// ```
/// use space_sum::parse_size;
///
/// let size = parse_size("1.5G").unwrap();
/// println!("Size in bytes: {size}");
/// ```
///
/// That will print Size in bytes: 1610612736
pub fn parse_size(expression: &str) -> Option<f64> {
    if expression.is_empty() {
        return Some(0.0);
    }

    // Assumes ZFS expressions like 3.70M
    let len = expression.len();
    let (number, unit) = expression
        .split_at(len - 1);

    match unit {
        "B" => parse_size_as_bytes(number, 0),
        "K" => parse_size_as_bytes(number, 1),
        "M" => parse_size_as_bytes(number, 2),
        "G" => parse_size_as_bytes(number, 3),
        "T" => parse_size_as_bytes(number, 4),
        _ => None,
    }
}

fn parse_size_as_bytes(expression: &str, power: i32) -> Option<f64> {
    let size: f64 = expression.parse().ok()?;

    Some(size * 1024f64.powi(power))
}

/// Print a size using B, K, M, G or T unit appropriately.
///
/// The size is rounded to three decimals.
///
/// # Arguments
///
/// * `size`: The space size, expressed in bytes
///
/// returns: A expression <size><unit>
///
/// # Examples
///
/// ```
/// use space_sum::human_readable_size;
///
/// let total_size = 1297991761.9200003;
/// let human_size = human_readable_size(total_size);
/// println!("Total size: {human_size}");
/// ```
///
/// That will print Total size: 1.209G
pub fn human_readable_size(size: f64) -> String {
    let scale = size.log(1024.0);

    if scale < 1.0 {
        format!("{}B", size)
    } else if scale < 2.0 {
        format!("{:.3}K", size / KB)
    } else if scale < 3.0 {
        format!("{:.3}M", size / MB)
    } else if scale < 4.0 {
        format!("{:.3}G", size / GB)
    } else {
        format!("{:.3}T", size / TB)
    }
}
