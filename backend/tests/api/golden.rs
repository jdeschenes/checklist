// Use an environment variable if that variable is set assume that the file must be written
// Otherwise read the file
// Loop through the response and compare the two files
//
use std::fs::File;
use std::io::BufWriter;

use serde_json::Value;
use similar::{Algorithm, TextDiff};
use time::{
    format_description::well_known::Rfc3339, macros::format_description, Date, OffsetDateTime,
};

const WRITE_ENVIRONMENT_VARIABLE: &str = "GOLDEN_OVERWRITE";

pub struct GoldenTest {
    test_dir: String,
}

impl GoldenTest {
    pub fn new() -> Self {
        Self {
            // TODO: Do the right interface to make this ergonomic
            // Add proper errors when checking the difference... Most of the stuff
            // Are irrelevant errors. Maybe we should panic?
            test_dir: "goldens".to_string(),
        }
    }
    pub fn new_with_dir(test_dir: &str) -> Self {
        Self {
            // TODO: Do the right interface to make this ergonomic
            // Add proper errors when checking the difference... Most of the stuff
            // Are irrelevant errors. Maybe we should panic?
            test_dir: test_dir.to_string(),
        }
    }

    pub fn check_diff_json(&self, test_name: &str, value: &Value) {
        match std::env::var(WRITE_ENVIRONMENT_VARIABLE) {
            Ok(_) => {
                self.write_golden_json(test_name, value);
            }
            Err(_) => {
                self.assert_golden_json(test_name, value);
            }
        }
    }
    pub fn assert_golden_json(&self, test_name: &str, value: &Value) {
        let file =
            File::open(std::path::Path::new(&self.test_dir).join(format!("{test_name}.json")))
                .expect("Failed to open file for reading");
        let data: serde_json::Value =
            serde_json::from_reader(file).expect("Failed to read data from json");
        let expected = serde_json::to_string_pretty(&data).expect("Failed to parsed expected data");
        let mut filtered_value = value.clone();
        dummify(&mut filtered_value);
        let gotten_value =
            serde_json::to_string_pretty(&filtered_value).expect("Failed to parsed gotten data");
        if data != filtered_value {
            let diff = TextDiff::configure()
                .algorithm(Algorithm::Patience)
                .diff_lines(&gotten_value, &expected)
                .unified_diff()
                .to_string();
            panic!("Data is different\n{}", diff);
        }
    }

    pub fn write_golden_json(&self, test_name: &str, value: &Value) {
        std::fs::create_dir_all(&self.test_dir).expect("Failed to create test folder");
        let file =
            File::create(std::path::Path::new(&self.test_dir).join(format!("{test_name}.json")))
                .expect("Failed to open file for writing");
        let mut writer = BufWriter::new(file);
        let mut filtered_value = value.clone();
        dummify(&mut filtered_value);
        serde_json::to_writer_pretty(&mut writer, &filtered_value)
            .expect("Failed to write json data to file");
    }
}

fn dummify(value: &mut Value) {
    let date_description = format_description!("[year]-[month]-[day]");
    match value {
        Value::String(ref mut s) => {
            // We can easily add new "types that we want to dummify
            if s.parse::<uuid::Uuid>().is_ok() {
                // This is the dummy replacement
                *s = "00000000-0000-0000-0000-000000000000".to_string();
                return;
            }
            if OffsetDateTime::parse(s, &Rfc3339).is_ok() {
                *s = "2023-02-01T00:00:00.123456Z".to_string();
                return;
            }
            if Date::parse(s, &date_description).is_ok() {
                *s = "2020-10-01".to_string();
                return;
            }
        }
        Value::Array(a) => {
            for element in a {
                dummify(element);
            }
        }
        Value::Object(o) => {
            for (_key, val) in o {
                dummify(val);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use rand::distr::{Alphanumeric, SampleString};

    use super::GoldenTest;

    #[test]
    fn test_golden_identical() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "empty": "",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // Write the file
        golden.write_golden_json(&test_name, &value);
        // Now the comparison should work
        golden.assert_golden_json(&test_name, &value);
    }

    #[test]
    #[should_panic(expected = "Data is different")]
    fn test_golden_different() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "empty": "",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // Write the file
        golden.write_golden_json(&test_name, &value);
        let value2 = serde_json::json!({
            "number": 2,
        });
        golden.assert_golden_json(&test_name, &value2);
    }

    #[test]
    #[should_panic(expected = "Failed to open file")]
    fn test_golden_does_not_exist() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "empty": "",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // This should fail since the file does not exist
        golden.assert_golden_json(&test_name, &value);
    }

    #[test]
    fn test_should_replace_uuid() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "uuid": uuid::Uuid::new_v4().to_string(),
            "list": [uuid::Uuid::new_v4().to_string()],
            "object": {
                "uuid": uuid::Uuid::new_v4().to_string(),
            },
        });
        golden.write_golden_json(&test_name, &value);
        let value = serde_json::json!({
            "uuid": uuid::Uuid::new_v4().to_string(),
            "list": [uuid::Uuid::new_v4().to_string()],
            "object": {
                "uuid": uuid::Uuid::new_v4().to_string(),
            },
        });
        golden.assert_golden_json(&test_name, &value);
    }

    #[test]
    fn test_should_replace_datetime() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "datetime": "2023-02-01T00:00:00.123456Z",
            "list": ["2023-02-01T04:00:10.123456Z"],
            "object": {
                "datetime": "2023-10-11T23:14:59.123456Z",
            },
        });
        golden.write_golden_json(&test_name, &value);
        let value = serde_json::json!({
            "datetime": "2023-02-01T20:00:00.123456Z",
            "list": ["2023-02-01T00:00:30.123456Z"],
            "object": {
                "datetime": "2023-02-01T10:00:00.123456Z",
            },
        });
        golden.assert_golden_json(&test_name, &value);
    }

    #[test]
    fn test_should_replace_date() {
        let temp_dir = std::env::temp_dir();
        let test_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let golden = GoldenTest::new_with_dir(temp_dir.to_str().unwrap());
        let value = serde_json::json!({
            "date": "2015-01-02",
            "list": ["2018-02-14"],
            "object": {
                "date": "2019-04-30",
            },
        });
        golden.write_golden_json(&test_name, &value);
        let value = serde_json::json!({
            "date": "2020-10-01",
            "list": ["2023-02-01"],
            "object": {
                "date": "2030-12-24",
            },
        });
        golden.assert_golden_json(&test_name, &value);
    }
}
