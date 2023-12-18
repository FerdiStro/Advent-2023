use std::io;
use std::collections::HashMap;
use std::num::ParseIntError;


// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green



fn main() {
    let max_green = 13;
    let max_red = 12;
    let max_blue = 14;


    let mut games = read_input();


    let mut non_valid_games_id: Vec<String> = Vec::new();


    for game in games {
        let id = game.split(':').next().and_then(|s| s.split("Game ").nth(1)).map_or("", |s| s.trim()).to_string();
        let rounds = game.split(":").nth(1).map_or("", |s| s.trim()).split(";");

        println!("Game_Id: {}", id);

        for round in rounds {
            for pic in round.split(',') {
                if pic.contains("green") {
                    let num: Result<i32, _> = pic.split("green").next().map_or("", |s| s.trim()).replace(" ", "").parse();
                    match num {
                        Ok(num) => {
                            if num > max_green {
                                non_valid_games_id.push(id.clone());
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                    if pic.contains("red") {
                        let num: Result<i32, _> = pic.split("red").next().map_or("", |s| s.trim()).replace(" ", "").parse();
                        match num {
                            Ok(num) => {
                                if num > max_red {
                                    non_valid_games_id.push(id.clone());
                                    break;
                                }
                            }
                            _ => {}
                        }
                        if pic.contains("blue") {
                            let num: Result<i32, _> = pic.split("blue").next().map_or("", |s| s.trim()).replace(" ", "").parse();
                            match num {
                                Ok(num) => {
                                    if num > max_blue {
                                        non_valid_games_id.push(id.clone());
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    println!("{}", round);

            }



            println!("nonVaild: ");
            for x in &non_valid_games_id{
                println!("{}", x);
            }
            println!("solution_1: {}", calculate_solution(&non_valid_games_id));


            // only 12 red cubes, 13 green cubes, and 14 blue cubes
        }
    }


    fn calculate_solution (list :&Vec<String>) -> i32 {
        let mut return_int: i32  = 0;
        for x in list{
            let i: Result<i32, _> = x.parse();
            match i{
                Ok(x) =>{
                    return_int += x;
                }
                _ => {}
            }
        }
        return  return_int;
    }

    fn read_input() -> Vec<String> {
        let mut input = String::new();
        let mut run_input: bool = true;
        let mut input_list: Vec<String> = Vec::new();

        while run_input == true {
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed_input = input.trim().to_string();
            match trimmed_input.as_str() {
                "stop" => run_input = false,
                _ => input_list.push(trimmed_input.clone()),
            }
            input.clear();
        }
        return input_list;
    }
}