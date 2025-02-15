// Use an environment variable if that variable is set assume that the file must be written
// Otherwise read the file
// Loop through the response and compare the two files
//
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use serde_json::Value;
use similar::{Algorithm, TextDiff};

const WRITE_ENVIRONMENT_VARIABLE: &str = "GOLDEN_OVERWRITE";

pub struct GoldenTest {
    test_file: String,
}

impl GoldenTest {
    pub fn new(test_file: &Path) -> Self {
        Self {
            // TODO: Do the right interface to make this ergonomic
            // Add proper errors when checking the difference... Most of the stuff
            // Are irrelevant errors. Maybe we should panic?
            test_file: test_file.to_str().unwrap().to_string(),
        }
    }

    pub fn check_diff(&self, value: &Value) {
        match std::env::var(WRITE_ENVIRONMENT_VARIABLE) {
            Ok(_) => {
                self.write_golden(value);
            }
            Err(_) => {
                self.assert_golden(value);
            }
        }
    }
    pub fn assert_golden(&self, value: &Value) {
        println!("Attempt to read file");
        let file = File::open(&self.test_file).expect("Failed to open file for reading");
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

    pub fn write_golden(&self, value: &Value) {
        println!("Attempt to write file");
        let file = File::create(&self.test_file).expect("Failed to open file for writing");
        let mut writer = BufWriter::new(file);
        let mut filtered_value = value.clone();
        dummify(&mut filtered_value);
        serde_json::to_writer_pretty(&mut writer, &filtered_value)
            .expect("Failed to write json data to file");
    }
}

fn dummify(value: &mut Value) {
    match value {
        Value::String(ref mut s) => {
            // We can easily add new "types that we want to dummify
            if s.parse::<uuid::Uuid>().is_ok() {
                // This is the dummy replacement
                *s = "00000000-0000-0000-0000-000000000000".to_string();
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
        let file_name = temp_dir.join(format!(
            "{}.json",
            Alphanumeric.sample_string(&mut rand::rng(), 16)
        ));
        let golden = GoldenTest::new(&file_name);
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // Write the file
        golden.write_golden(&value);
        // Now the comparison should work
        golden.assert_golden(&value);
    }

    #[test]
    #[should_panic(expected = "Data is different")]
    fn test_golden_different() {
        let temp_dir = std::env::temp_dir();
        let file_name = temp_dir.join(format!(
            "{}.json",
            Alphanumeric.sample_string(&mut rand::rng(), 16)
        ));
        let golden = GoldenTest::new(&file_name);
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // Write the file
        golden.write_golden(&value);
        let value2 = serde_json::json!({
            "number": 2,
        });
        golden.assert_golden(&value2);
    }

    #[test]
    #[should_panic(expected = "Failed to open file")]
    fn test_golden_does_not_exist() {
        let temp_dir = std::env::temp_dir();
        let file_name = temp_dir.join(format!(
            "{}.json",
            Alphanumeric.sample_string(&mut rand::rng(), 16)
        ));
        let golden = GoldenTest::new(&file_name);
        let value = serde_json::json!({
            "number": 1,
            "text": "text",
            "null": null,
            "bool": true,
            "array": ["text"],
            "object": {
                "element": false
            }

        });
        // This should fail since the file does not exist
        golden.assert_golden(&value);
    }

    #[test]
    fn test_should_replace_uuid() {
        let temp_dir = std::env::temp_dir();
        let file_name = temp_dir.join(format!(
            "{}.json",
            Alphanumeric.sample_string(&mut rand::rng(), 16)
        ));
        let golden = GoldenTest::new(&file_name);
        let value = serde_json::json!({
            "uuid": uuid::Uuid::new_v4().to_string(),
            "list": [uuid::Uuid::new_v4().to_string()],
        });
        golden.write_golden(&value);
        let value = serde_json::json!({
            "uuid": uuid::Uuid::new_v4().to_string(),
            "list": [uuid::Uuid::new_v4().to_string()],
        });
        golden.assert_golden(&value);
    }
}
