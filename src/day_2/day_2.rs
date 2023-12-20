use crate::input_reader::input_reader as reader;

pub fn solution_2() {
    let max_green = 13;
    let max_red = 12;
    let max_blue = 14;

    let mut games = reader::read_input();
    let mut valid_games_id: Vec<String> = Vec::new();
    let mut power_games: Vec<String> =  Vec::new();


    for game in games {
        let id = game.split(':').next().and_then(|s| s.split("Game ").nth(1)).map_or("", |s| s.trim()).to_string();
        let rounds = game.split(":").nth(1).map_or("", |s| s.trim()).split(";");
        let mut non_valid = false;

        let mut max_round_green = 0 ;
        let mut max_round_red = 0 ;
        let mut max_round_blue = 0 ;

        for round in rounds {
            for pic in round.split(',') {
                if pic.contains("green") {
                    let num: Result<i32, _> = pic.split("green").next().map_or("", |s| s.trim()).replace(" ", "").parse();
                    match num {
                        Ok(num) => {
                            if num > max_green {
                                non_valid = true;
                            }
                            if num > max_round_green {
                                max_round_green  = num;
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
                                non_valid = true;
                            }
                            if num > max_round_red {
                                max_round_red =  num;
                            }
                        }
                        _ => {}
                    }
                }
                if pic.contains("blue") {
                    let num: Result<i32, _> = pic.split("blue").next().map_or("", |s| s.trim()).replace(" ", "").parse();
                    match num {
                        Ok(num) => {
                            if num > max_blue {
                                non_valid = true;
                            }
                            if num > max_round_blue{
                                max_round_blue = num;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if !non_valid {
            valid_games_id.push(id.clone());
        }
        power_games.push((&max_round_blue * &max_round_red * &max_round_green).to_string())
    }
    println!("solution_1: {}", calculate_solution(&valid_games_id));
    println!("solution_2: {}", calculate_solution(&power_games));
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
    return_int
}