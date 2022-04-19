use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

use crate::HashType;

pub struct ReturnStructure {
    pub hash_type: HashType,
    pub hash_sum: String,
    pub word_generation_preferences: Vec<bool>,
}

pub fn get() -> ReturnStructure {
    let selections = &["sha256", "sha512"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select hash type")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let hash_type = match selections[selection] {
        "sha256" => HashType::Sha256,
        "sha512" => HashType::Sha512,
        _ => {
            println!("{}", "Non-existent option!".red());
            std::process::exit(1);
        }
    };

    let hash_sum: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Hash")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if (hash_type.clone() == HashType::Sha256 && input.len() == 64)
                    || (hash_type.clone() == HashType::Sha512 && input.len() == 128)
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
        .unwrap();

    let mut word_generation_preferences = vec![false; 4];

    let multiselected = &[
        "Lowercase",
        "Uppercase",
        "Numbers",
        "Spectial symbols (!\"#$%&'()*+,-./:;<=>?@^_`{|}~)",
    ];
    let defaults = &[true, false, false, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Use for word generation")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("{}", "You need to select at least one option!".red());
        std::process::exit(1);
    } else {
        for selection in selections {
            match multiselected[selection] {
                "Lowercase" => word_generation_preferences[0] = true,
                "Uppercase" => word_generation_preferences[1] = true,
                "Numbers" => word_generation_preferences[2] = true,
                "Spectial symbols (!\"#$%&'()*+,-./:;<=>?@^_`{|}~)" => {
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
    }
}
