use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use glob::glob;
use serde_yaml::{Value, Mapping};
use serde_yaml::Mapping as YamlMapping;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the hashmap to store words by first letter
    let mut word_groups: HashMap<char, Vec<YamlMapping>> = HashMap::new();

    // Traverse current directory and all subdirectories looking for .yaml files
    for entry in glob("**/*.yaml")? {
        let path = entry?;
        // Skip files inside the "word" directory
        if path.starts_with("word") {
            continue;
        }
        if path.is_file() {
            // Read and parse YAML file
            let file_contents = fs::read_to_string(&path)?;
            let yaml_data: Value = serde_yaml::from_str(&file_contents)?;

            // Get the word (key of YAML object)
            if let Some(map) = yaml_data.as_mapping() {
                if let Some((key, value)) = map.iter().next() {
                    if let Some(word) = key.as_str() {
                        if let Some(first_letter) = word.chars().next() {
                            let mut new_mapping = YamlMapping::new();
                            new_mapping.insert(key.clone(), value.clone());
                            // Insert into the appropriate list by first letter
                            word_groups.entry(first_letter).or_insert(vec![]).push(new_mapping);
                        }
                    }
                }
            }
        }
    }

    // Create a folder called "word" in the root directory to store the output
    let output_dir = Path::new("word");
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir(output_dir)?;

    // Write each list of words into a YAML file based on the first letter
    for (letter, words) in word_groups.iter() {
        let file_path = output_dir.join(format!("{}.yaml", letter));
        let mut file = File::create(&file_path)?;
        for word in words.iter() {
            let yaml_string = serde_yaml::to_string(word)?;
            file.write_all(yaml_string.as_bytes())?;
            file.write_all(b"\n")?; // Add a newline between YAML documents
        }
    }

    Ok(())
}

// Add to your dependencies in Cargo.toml:
// serde = { version = "1.0", features = ["derive"] }
// serde_yaml = "0.8"
// glob = "0.3"
