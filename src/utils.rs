use crate::study_cycle::Subject;

pub fn get_biggest_string_len(strings: Vec<String>) -> usize {
    if strings.is_empty() {
        return 0;
    }
    let mut temp_vec: Vec<String> = strings.clone();
    temp_vec.sort_by(|a, b| b.len().cmp(&a.len()));
    temp_vec[0].len()
}

pub fn display_table(subjects: Vec<Subject>) {
    let subject_names: Vec<String> = subjects.iter().map(|s| s.name.clone()).collect();
    let headers: Vec<String> = vec![
        "NAME".to_owned(),
        "STUDIED HOURS".to_owned(),
        "MAX STUDY HOURS".to_owned(),
    ];
    let biggest_cell = get_biggest_string_len(subject_names);
    let biggest_header = get_biggest_string_len(headers);
    if biggest_cell > biggest_header {
        println!(
            "{0: <biggest_cell$} | {1: <biggest_cell$} | {2: <biggest_cell$}",
            "NAME", "STUDIED HOURS", "MAX STUDY HOURS"
        );
        for subject in subjects {
            println!(
                "{0: <biggest_cell$} | {1: <biggest_cell$} | {2: <biggest_cell$}",
                subject.name, subject.studied_hours, subject.max_study_hours
            );
        }
    } else {
        println!(
            "{0: <biggest_header$} | {1: <biggest_header$} | {2: <biggest_header$}",
            "NAME", "STUDIED HOURS", "MAX STUDY HOURS"
        );
        for subject in subjects {
            println!(
                "{0: <biggest_header$} | {1: <biggest_header$} | {2: <biggest_header$}",
                subject.name,
                format!("{}h", subject.studied_hours),
                format!("{}h", subject.max_study_hours),
            );
        }
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
