pub fn get_biggest_string_len(strings: Vec<String>) -> usize {
    if strings.is_empty() {
        return 0;
    }
    let mut temp_vec: Vec<String> = strings.clone();
    temp_vec.sort_by(|a, b| b.len().cmp(&a.len()));
    temp_vec[0].len()
}
