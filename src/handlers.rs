use crate::model::{read_json_database_file, udpate_json_database};

pub fn study_subject(name: String) {
    let mut db = read_json_database_file();

    let subject = db.subjects.iter_mut().find(|s| s.name == name);

    match subject {
        Some(subject) => {
            if subject.studied_hours >= subject.max_study_hours {
                println!(
                    "locked subject: {}/{}h studied already.",
                    subject.studied_hours, subject.max_study_hours
                );
                return;
            }

            subject.studied_hours += 1;

            udpate_json_database(db);

            println!("studied!")
        }
        None => println!("subject '{}' not found.", name),
    }
}
