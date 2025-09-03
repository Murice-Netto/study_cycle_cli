use crate::study_cycle::Subject;

pub fn get_biggest_string_len(strings: &[&str]) -> usize {
    strings.iter().map(|s| s.len()).max().unwrap_or(0)
}

pub fn display_table(subjects: &[Subject]) {
    let name_col_len = subjects
        .iter()
        .map(|s| s.name.len())
        .max()
        .unwrap_or(0)
        .max("NAME".len());
    let studied_col_len = "STUDIED HOURS".len();
    let max_col_len = "STUDIED HOURS".len();
    println!(
        "{0: <name_col_len$} | {1: <studied_col_len$} | {2: <max_col_len$}",
        "NAME", "STUDIED HOURS", "MAX STUDY HOURS"
    );
    for subject in subjects {
        println!(
            "{0: <name_col_len$} | {1: <studied_col_len$} | {2: <max_col_len$}",
            subject.name, subject.studied_hours, subject.max_study_hours
        );
    }
}

pub fn display_table_with_progress_bar(subjects: &[Subject]) {
    if subjects.is_empty() {
        println!("No subjects were found.");
        return;
    }
    let name_width = subjects
        .iter()
        .map(|s| s.name.len())
        .max()
        .unwrap_or(0)
        .max("NAME".len());
    let progress_width = subjects
        .iter()
        .map(|s| s.max_study_hours as usize)
        .max()
        .unwrap_or(0)
        .max("PROGRESS BAR".len());
    let hours_width = subjects
        .iter()
        .map(|s| format!("{}/{}h", s.studied_hours, s.max_study_hours).len())
        .max()
        .unwrap_or(0)
        .max("HOURS".len());
    println!(
        "{0: <name_width$} | {1: <progress_width$} | {2: <hours_width$}",
        "NAME", "PROGRESS BAR", "HOURS"
    );
    for subject in subjects {
        let progress_bar =
            get_study_hours_progress_bar(subject.studied_hours, subject.max_study_hours);
        let hours_text = format!("{}/{}h", subject.studied_hours, subject.max_study_hours);
        println!(
            "{0: <name_width$} | {1: <progress_width$} | {2: <hours_width$}",
            subject.name, progress_bar, hours_text
        );
    }
}

pub fn get_study_hours_progress_bar(studied_hours: u8, max_study_hours: u8) -> String {
    const EMPTY: char = '□';
    const FULL: char = '■';
    let full_part: String = std::iter::repeat(FULL)
        .take(studied_hours as usize)
        .collect();
    let empty_parts: String = std::iter::repeat(EMPTY)
        .take((max_study_hours - studied_hours) as usize)
        .collect();
    format!("{}{}", full_part, empty_parts)
}
