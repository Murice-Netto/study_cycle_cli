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

pub fn display_table_with_progress_bar(subjects: Vec<Subject>) {
    let subject_names: Vec<String> = subjects.iter().map(|s| s.name.clone()).collect();
    let mut temp_subjects = subjects.clone();
    let headers: Vec<String> = vec![
        "NAME".to_owned(),
        "PROGRESS BAR".to_owned(),
        "HOURS".to_owned(),
    ];
    let biggest_cell = get_biggest_string_len(subject_names);
    let biggest_header = get_biggest_string_len(headers);
    temp_subjects.sort_by_key(|s| s.max_study_hours);
    let biggest_progress_bar = temp_subjects
        .last()
        .expect("failed to get last subject")
        .clone();
    let mut possible_values_to_space_columns = vec![
        biggest_cell,
        biggest_header,
        biggest_progress_bar.max_study_hours as usize,
    ];
    possible_values_to_space_columns.sort();
    let space_between_columns = possible_values_to_space_columns
        .last()
        .expect("failed to get the last possible vlaes to space columns")
        .clone();
    println!(
        "{0: <space_between_columns$} | {1: <space_between_columns$} | {2: <space_between_columns$}",
        "NAME", "PROGRESS BAR", "HOURS"
    );
    for subject in subjects {
        println!(
            "{0: <space_between_columns$} | {1: <space_between_columns$} | {2: <space_between_columns$}",
            subject.name,
            get_study_hours_progress_bar(
                subject.studied_hours.into(),
                subject.max_study_hours.into()
            ),
            format!("{}/{}h", subject.studied_hours, subject.max_study_hours)
        );
    }
}

pub fn get_study_hours_progress_bar(studied_hours: i64, max_study_hours: i64) -> String {
    const EMPTY: char = '□';
    const FULL: char = '■';
    let remaning_hours = max_study_hours - studied_hours;
    format!(
        "{}{}",
        FULL.to_string().repeat(studied_hours as usize),
        EMPTY.to_string().repeat(remaning_hours as usize)
    )
}
