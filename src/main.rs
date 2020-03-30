use std::path::Path;
use std::str::FromStr;

use err_context::AnyError;
use percent_encoding::percent_decode_str;
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Debug, Serialize)]
#[serde(default, rename_all = "kebab-case")]
struct Group {
    name: String,
    id: u8,
    people: Vec<Person>,
}

#[derive(Debug, Serialize)]
#[serde(default, rename_all = "camelCase")]
struct Person {
    name: String,
    surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
}

fn get_files(root_dir: &str) -> Vec<(String, u8, String)> {
    let re = regex::Regex::new(r"data_(\d+)_(.*)\.csv").unwrap();

    WalkDir::new(root_dir)
        .into_iter().filter_map(|e| {
        if let Ok(e) = e {
            let name = e.file_name().to_str().unwrap();

            re.captures_iter(name).next().map(|capt| {
                ((&capt[2]).to_string(), u8::from_str(&capt[1]).unwrap(), format!("{}/{}", root_dir, name).to_string())
            })
        } else { None }
    }).collect()
}

fn photo_exists(root_dir: &str, name: &str) -> bool {
    Path::new(format!("{}/photos/{}.jpeg", root_dir, name).as_str()).exists()
}

fn extract_name(linkedin_url: &str) -> Option<String> {
    let linkedin_url = percent_decode_str(linkedin_url).decode_utf8().unwrap().to_string();

    let re = regex::Regex::new(r"https://www.linkedin.com/in/(.*)/").unwrap();

    re.captures(linkedin_url.as_str()).and_then(|m| {
        m.get(1)
    }).map(|m| m.as_str().to_string())
}

fn run(root_dir: &str, photo_url_prefix: &str) -> Result<(), AnyError> {
    let files = get_files(root_dir);

    let mut groups: Vec<Group> = Vec::new();

    for (group_name, group_id, file_path) in files.into_iter() {
        let mut reader = csv::ReaderBuilder::new().from_path(file_path).unwrap();

        let mut people: Vec<Person> = Vec::new();

        for record in reader.records() {
            let record = record?;

            let (linkedin, photo_url) = if let Some(url) = record.get(1) {
                if url != "" {
                    let photo = extract_name(url).and_then(|name| {
                        if !photo_exists(root_dir, &name) {
                            println!("Name without photo: {}", name);
                            // webbrowser::open(format!("{}/detail/photo", url).as_str()).unwrap();
                            None
                        } else { Some(format!("{}{}.jpeg", photo_url_prefix, name)) }
                    });

                    (Some(url.to_string()), photo)
                } else {
                    (None, None)
                }
            } else { (None, None) };

            match record.get(0).unwrap().split(" ").collect::<Vec<_>>().as_slice() {
                &[name, surname, ..] if name != "" => {
                    let person = Person {
                        name: name.to_string(),
                        surname: surname.to_string(),
                        linkedin,
                        photo_url,
                    };

                    people.push(person);
                }
                &[name, ..] if name != "" => {
                    let person = Person {
                        name: name.to_string(),
                        surname: "".to_string(),
                        linkedin,
                        photo_url,
                    };

                    people.push(person);
                }
                _ => ()
            }
        }

        groups.push(Group {
            name: group_name,
            id: group_id,
            people,
        })
    }

    groups.sort_by_key(|g| g.id);

    let json = serde_json::to_string(groups.as_slice()).unwrap();

    println!("{}", json);

    std::fs::write("output.json", json).unwrap();

    Ok(())
}

fn main() -> Result<(), AnyError> {
    let args: Vec<String> = std::env::args().collect();

    run(&args[1], &args[2])
}
