mod models;
use crate::models::player::Player;

use std::io::{ stdin, stdout, Write };

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let players_count = get_players_count();
    let mut players = get_players(players_count);

    loop {
        println!("Randomizing teams..\n");
        stdout().flush().unwrap();

        let teams = get_randomized_teams(&mut players);
        print_teams(teams);

        let mut finished_randomizing = false;

        loop {
            print!("\nDo you want to randomize more teams? (Y/n): ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Wrong input!");

            match input.trim().chars().next() {
                Some('Y') | Some('y') | None => {
                    break;
                }
                Some('N') | Some('n') => {
                    finished_randomizing = true;
                    println!("No more teams will be randomized.");
                    break;
                }
                _ => println!("Invalid input, please enter Y or n"),
            }
        }

        if finished_randomizing {
            break;
        }
    }

    println!("Press enter to exit application..");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Wrong input!");
}

fn get_players_count() -> u8 {
    let count: u8;

    loop {
        print!("How many players will attend the game (10-14): ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Wrong input!");

        let players_count: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number (0-255).");
                continue;
            }
        };

        if players_count >= 10 && players_count <= 14 {
            count = players_count;
            break;
        } else {
            println!("Number of players should be between 10 and 14.");
        }
    }

    return count;
}

fn get_players(count: u8) -> Vec<Player> {
    let count = count as usize;

    println!(
        "In the next section you will need to enter player names and ability to run (1-10) in format (name,5)."
    );
    stdout().flush().unwrap();

    let mut players: Vec<Player> = Vec::with_capacity(count);

    for player in 0..count {
        loop {
            let mut input = String::new();

            print!("{}.", player + 1);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("wrong input!");

            // Trim the input and split it
            let parts: Vec<&str> = input.trim().split(",").collect();

            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let score = parts[1].trim();

                match score.parse::<i8>() {
                    Ok(num) => {
                        if num < 1 || num > 10 {
                            println!("Ability to run should be between 1 and 10 (1-10).");
                        } else {
                            players.push(Player { name, score: num });
                            break;
                        }
                    }
                    Err(_) => {
                        println!("The format is incorrect. Please try again. eg.: Name,5");
                    }
                }
            } else {
                println!("The format is incorrect. Please try again. eg.: Name,5");
            }
        }
    }

    return players;
}

fn get_randomized_teams(players: &mut Vec<Player>) -> (Vec<Player>, Vec<Player>) {
    // Shuffle players
    let mut rng = thread_rng();
    players.shuffle(&mut rng);

    // Sort players by score in descending order
    players.sort_by(|a, b| b.score.cmp(&a.score));

    let mut team_a: Vec<Player> = Vec::new();
    let mut team_b: Vec<Player> = Vec::new();
    let mut score_a: i8 = 0;
    let mut score_b: i8 = 0;

    for player in players {
        let player_clone = player.clone();
        if team_a.len() == team_b.len() {
            if score_a <= score_b {
                score_a += player.score;
                team_a.push(player_clone);
            } else {
                score_b += player.score;
                team_b.push(player_clone);
            }
        } else if team_a.len() < team_b.len() {
            score_a += player.score;
            team_a.push(player_clone);
        } else {
            score_b += player.score;
            team_b.push(player_clone);
        }
    }

    (team_a, team_b)
}

fn print_teams(teams: (Vec<Player>, Vec<Player>)) {
    let (team_a, team_b) = teams;

    // Display Team 1
    println!("Team A:");
    for (index, player) in team_a.iter().enumerate() {
        println!("{}. {}", index + 1, player.name);
    }

    // Add a blank line for better readability
    println!();

    // Display Team 2
    println!("Team B:");
    for (index, player) in team_b.iter().enumerate() {
        println!("{}. {}", index + 1, player.name);
    }
}
