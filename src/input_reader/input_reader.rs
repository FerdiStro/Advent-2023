use std::io;
pub fn read_input() -> Vec<String> {
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



