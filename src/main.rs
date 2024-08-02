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
    class: String,
    stats: Stats,
    advantages: Vec<String>,
    disadvantages: Vec<String>,
    skills: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    health: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    crit_rate: f32,
    crit_damage: f32,
    effect_hit_rate: f32,
    effect_resistance: f32,
    dual_attack_rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Skill {
    name: String,
    description: String,
    tags: Vec<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "character_info_system")]
struct Opt {
    /// JSON file containing character data
    #[structopt(short, long, default_value = "characters.json")]
    file: String,

    /// Display character details by name
    #[structopt(short, long)]
    character: Option<String>,

    /// Find characters by skill tag
    #[structopt(short, long)]
    skill_tag: Option<String>,

    /// Perform meta search for disadvantages of selected characters
    #[structopt(short, long)]
    meta: Option<Vec<String>>,

    /// Enable interactive mode
    #[structopt(short, long)]
    interactive: bool,
}

fn load_characters(file_path: &str) -> io::Result<Vec<Character>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let characters: Vec<Character> = from_reader(reader)?;
    Ok(characters)
}

fn display_character(character: &Character) {
    println!("{}", format!("Name: {}", character.name).cyan());
    println!("{}", format!("Class: {}", character.class).yellow());
    println!("{}", "Stats:".green());
    println!("{}", format!("  Health: {}", character.stats.health).blue());
    println!("{}", format!("  Attack: {}", character.stats.attack).blue());
    println!("{}", format!("  Defense: {}", character.stats.defense).blue());
    println!("{}", format!("  Speed: {}", character.stats.speed).blue());
    println!("{}", format!("  Crit Rate: {}", character.stats.crit_rate).blue());
    println!("{}", format!("  Crit Damage: {}", character.stats.crit_damage).blue());
    println!("{}", format!("  Effect Hit Rate: {}", character.stats.effect_hit_rate).blue());
    println!("{}", format!("  Effect Resistance: {}", character.stats.effect_resistance).blue());
    println!("{}", format!("  Dual Attack Rate: {}", character.stats.dual_attack_rate).blue());
    println!("{}", format!("Advantages: {:?}", character.advantages).magenta());
    println!("{}", format!("Disadvantages: {:?}", character.disadvantages).red());
    println!("{}", "Skills:".green());
    for skill in &character.skills {
        println!("{}", format!("  Name: {}", skill.name).cyan());
        println!("{}", format!("  Description: {}", skill.description).blue());
        println!("{}", format!("  Tags: {:?}", skill.tags).yellow());
    }
}

fn find_disadvantages(characters: &Vec<Character>, selected: Vec<&str>) -> Vec<String> {
    let mut disadvantage_counts: HashMap<String, usize> = HashMap::new();

    for character in characters {
        if selected.contains(&character.name.as_str()) {
            for disadvantage in &character.disadvantages {
                *disadvantage_counts.entry(disadvantage.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut disadvantages: Vec<(String, usize)> = disadvantage_counts.into_iter().collect();

    // ソート: 出現回数が高い順
    disadvantages.sort_by(|a, b| b.1.cmp(&a.1));

    // ソートされた結果からキャラクター名のみを抽出
    disadvantages.into_iter().map(|(name, _)| name).collect()
}

fn find_characters_by_skill_tag<'a>(characters: &'a Vec<Character>, tag: &'a str) -> Vec<&'a Character> {
    characters.iter().filter(|character| {
        character.skills.iter().any(|skill| skill.tags.contains(&tag.to_string()))
    }).collect::<Vec<&Character>>()
}

fn interactive_mode(characters: &Vec<Character>) {
    loop {
        println!("{}", "Select an option:".green());
        println!("{}", "1. Display character details".cyan());
        println!("{}", "2. Find characters by skill tag".cyan());
        println!("{}", "3. Meta search for disadvantages".cyan());
        println!("{}", "4. Exit".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter character name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                if let Some(character) = characters.iter().find(|c| c.name == name) {
                    display_character(character);
                } else {
                    println!("Character not found.");
                }
            }
            2 => {
                println!("Enter skill tag:");
                let mut tag = String::new();
                io::stdin().read_line(&mut tag).expect("Failed to read line");
                let tag = tag.trim();
                let characters_with_skill = find_characters_by_skill_tag(&characters, tag);
                for character in characters_with_skill {
                    println!("{}", character.name);
                }
            }
            3 => {
                println!("Enter character names (comma separated):");
                let mut names = String::new();
                io::stdin().read_line(&mut names).expect("Failed to read line");
                let names: Vec<&str> = names.trim().split(',').map(|s| s.trim()).collect();
                let disadvantages = find_disadvantages(&characters, names);
                println!("Disadvantages for selected characters:");
                for disadvantage in disadvantages {
                    println!("{}", disadvantage);
                }
            }
            4 => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let characters = load_characters(&opt.file)?;

    if opt.interactive {
        interactive_mode(&characters);
    } else {
        if let Some(name) = opt.character {
            if let Some(character) = characters.iter().find(|c| c.name == name) {
                display_character(character);
            } else {
                println!("Character not found.");
            }
        }

        if let Some(tag) = opt.skill_tag {
            let characters_with_skill = find_characters_by_skill_tag(&characters, &tag);
            for character in characters_with_skill {
                println!("{}", character.name);
            }
        }

        if let Some(selected) = opt.meta {
            let disadvantages = find_disadvantages(&characters, selected.iter().map(String::as_str).collect());
            println!("Disadvantages for selected characters:");
            for disadvantage in disadvantages {
                println!("{}", disadvantage);
            }
        }
    }

    Ok(())
}
