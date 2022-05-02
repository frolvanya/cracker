use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

use crate::HashType;

pub struct ReturnStructure {
    pub hash_type: HashType,
    pub hash_sum: String,
    pub word_generation_preferences: Vec<bool>,
    pub file_name: String,
}

pub fn get() -> ReturnStructure {
    let hash_selections = &["sha256", "sha512", "md5"];

    let hash_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select hash type")
        .default(0)
        .items(&hash_selections[..])
        .interact()
        .unwrap();

    let hash_type = match hash_selections[hash_selection] {
        "sha256" => HashType::Sha256,
        "sha512" => HashType::Sha512,
        "md5" => HashType::Md5,
        _ => unreachable!(),
    };

    let hash_sum = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Hash")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if (hash_type == HashType::Sha256 && input.len() == 64)
                    || (hash_type == HashType::Sha512 && input.len() == 128)
                    || (hash_type == HashType::Md5 && input.len() == 32)
                    || force.as_ref().map_or(false, |old| old == input)
                {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a valid hash sum! Type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap()
        .to_lowercase();

    let crack_selections = &["Generate words", "Use file"];

    let crack_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select crack type")
        .default(0)
        .items(&crack_selections[..])
        .interact()
        .unwrap();

    match crack_selections[crack_selection] {
        "Generate words" => {
            let mut word_generation_preferences = vec![false; 4];

            let word_multiselected = &[
                "lowercase",
                "uppercase",
                "numbers",
                "spectial symbols (!\"#$%&'()*+,-./:;<=>?@^_`{|}~)",
            ];
            let word_defaults = &[true, false, false, false];

            let word_selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Use for word generation")
                .items(&word_multiselected[..])
                .defaults(&word_defaults[..])
                .interact()
                .unwrap();

            if word_selections.is_empty() {
                println!("{}", "You need to select at least one option!".red());
                std::process::exit(1);
            } else {
                for word_selection in word_selections {
                    match word_multiselected[word_selection] {
                        "lowercase" => word_generation_preferences[0] = true,
                        "uppercase" => word_generation_preferences[1] = true,
                        "numbers" => word_generation_preferences[2] = true,
                        "spectial symbols (!\"#$%&'()*+,-./:;<=>?@^_`{|}~)" => {
                            word_generation_preferences[3] = true
                        }
                        _ => {}
                    };
                }
            }

            ReturnStructure {
                hash_type,
                hash_sum,
                word_generation_preferences,
                file_name: String::new(),
            }
        }
        "Use file" => {
            let file_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("File name")
                .default("rockyou.txt".to_string())
                .interact_text()
                .unwrap();

            ReturnStructure {
                hash_type,
                hash_sum,
                word_generation_preferences: Vec::new(),
                file_name,
            }
        }
        _ => unreachable!(),
    }
}
