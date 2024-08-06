use std::fs::File;
use std::io::{self, BufReader};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use structopt::StructOpt;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Character {
    name: String,
    alias: Vec<String>,
    advantages: Vec<String>,
    disadvantages: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "character_info_system")]
struct Opt {
    /// JSON file containing character data
    #[structopt(short, long, default_value = "characters.json")]
    file: String,

    /// Display character details by name or alias
    #[structopt(short, long)]
    character: Option<String>,

    /// Perform meta search for disadvantages of selected characters
    #[structopt(short, long)]
    meta: Option<Vec<String>>,

    /// List all characters
    #[structopt(short, long)]
    list: bool,
}

fn load_characters(file_path: &str) -> io::Result<Vec<Character>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let characters: Vec<Character> = from_reader(reader)?;
    Ok(characters)
}

fn display_character(character: &Character) {
    println!("{}", format!("Name: {}", character.name).cyan());
    println!("{}", format!("Alias: {:?}", character.alias).cyan());
    println!("{}", format!("Advantages: {:?}", character.advantages).magenta());
    println!("{}", format!("Disadvantages: {:?}", character.disadvantages).red());
}

fn display_character_list(characters: &Vec<Character>) {
    for character in characters {
        println!("{} ({:?})", character.name.cyan(), character.alias);
    }
}

fn find_disadvantages(characters: &Vec<Character>, selected: Vec<&str>) -> Vec<String> {
    let mut disadvantage_counts: HashMap<String, usize> = HashMap::new();

    for character in characters {
        if selected.contains(&character.name.as_str()) || character.alias.iter().any(|alias| selected.contains(&alias.as_str())) {
            for disadvantage in &character.disadvantages {
                *disadvantage_counts.entry(disadvantage.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut disadvantages: Vec<(String, usize)> = disadvantage_counts.into_iter().collect();

    // Sort by occurrence count in descending order
    disadvantages.sort_by(|a, b| b.1.cmp(&a.1));

    // Extract only the character names from the sorted result
    disadvantages.into_iter().map(|(name, _)| name).collect()
}

fn interactive_mode(characters: &Vec<Character>) {
    loop {
        println!("{}", "Select an option:".green());
        println!("{}", "1. Display character details".cyan());
        println!("{}", "2. Meta search for disadvantages".cyan());
        println!("{}", "3. List all characters".cyan());
        println!("{}", "4. Exit".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter character name or alias:");
                let mut name_or_alias = String::new();
                io::stdin().read_line(&mut name_or_alias).expect("Failed to read line");
                let name_or_alias = name_or_alias.trim();
                if let Some(character) = characters.iter().find(|c| c.name == name_or_alias || c.alias.contains(&name_or_alias.to_string())) {
                    display_character(character);
                } else {
                    println!("Character not found.");
                }
            }
            2 => {
                println!("Enter character names or aliases (comma separated):");
                let mut names_or_aliases = String::new();
                io::stdin().read_line(&mut names_or_aliases).expect("Failed to read line");
                let names_or_aliases: Vec<&str> = names_or_aliases.trim().split(',').map(|s| s.trim()).collect();
                let disadvantages = find_disadvantages(&characters, names_or_aliases);
                println!("Disadvantages for selected characters:");
                for disadvantage in disadvantages {
                    println!("{}", disadvantage);
                }
            }
            3 => {
                display_character_list(&characters);
            }
            4 => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let characters = load_characters(&opt.file)?;

    if opt.character.is_none() && opt.meta.is_none() && !opt.list {
        interactive_mode(&characters);
    } else {
        if let Some(name_or_alias) = opt.character {
            if let Some(character) = characters.iter().find(|c| c.name == name_or_alias || c.alias.contains(&name_or_alias)) {
                display_character(character);
            } else {
                println!("Character not found.");
            }
        }

        if let Some(selected) = opt.meta {
            let disadvantages = find_disadvantages(&characters, selected.iter().map(String::as_str).collect());
            println!("Disadvantages for selected characters:");
            for disadvantage in disadvantages {
                println!("{}", disadvantage);
            }
        }

        if opt.list {
            display_character_list(&characters);
        }
    }

    Ok(())
}
