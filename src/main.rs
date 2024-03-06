mod models;
use crate::models::player::Player;

use std::fs;
use std::io::{ stdin, stdout, Write };

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let input = read_from_input();

    let mut players = get_players(input);

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

fn read_from_input() -> String {
    let file_path = "src/assets/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn get_players(input: String) -> Vec<Player> {
    let players_input = input.split('\n').collect::<Vec<&str>>();
    let players_count = players_input.len();

    let mut players: Vec<Player> = Vec::with_capacity(players_count);

    for player in players_input {
        let player_data = player.split(',').collect::<Vec<&str>>();
        let player_name = player_data[0].to_string();
        let player_score = player_data[1].parse().expect("Score is not a valid number");
        players.push(Player { name: player_name, score: player_score });
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
