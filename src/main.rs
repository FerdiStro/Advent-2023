use std::io;


// fn find_and_replace_first<'a>(input: &'a str, ziffern: &[(&str, &str)]) -> Option<(usize, &'a str, &'a str)> {
//     for &(wort, ziffer) in ziffern {
//         if let Some(pos) = input.find(wort) {
//             return Some((pos, wort, ziffer));
//         }
//     }
//     None
// }

// fn find_and_replace_last<'a>(input: &'a str, ziffern: &[(&str, &str)]) -> Option<(usize, &'a str, &'a str)> {
//     for &(wort, ziffer) in ziffern.iter().rev() {
//         if let Some(pos) = input.rfind(wort) {
//             return Some((pos, wort, ziffer));
//         }
//     }
//     None
// }

// fn map_string_int(mut line: String) -> String {
//     let list = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

//     let ziffern = [
//         ("one",   "1"),
//         ("two",   "2"),
//         ("three", "3"),
//         ("four",  "4"),
//         ("five",  "5"),
//         ("six",   "6"),
//         ("seven", "7"),
//         ("eight", "8"),
//         ("nine",  "9"),
//     ];

//      // Klonen, um das Borrow Checker-Problem zu umgehen
//     let mut modified_string = input_string.clone();

//     // Ersetze das gefundene Wort an der ersten Position durch die entsprechende Ziffer
//     if let Some((erste_pos, wort, ziffer)) = find_and_replace_first(&input_string, &ziffern) {
//         modified_string.replace_range(erste_pos..(erste_pos + wort.len()), ziffer);
//     }

//     // Ersetze das gefundene Wort an der letzten Position durch die entsprechende Ziffer
//     if let Some((letzte_pos, wort, ziffer)) = find_and_replace_last(&input_string, &ziffern) {
//         modified_string.replace_range(letzte_pos..(letzte_pos + wort.len()), ziffer);
//     }

//     println!("LINE: {}", line);
//     return line
// }


fn main() {
 
    let mut list  = read_input();
    let mut result : usize = 0;
    



    for line in &mut list {


        // *line = map_string_int(line.clone());

       

        




        let parts =  line.chars();

        let mut line_num = String::new();

        for part  in parts {
            if !part.is_alphabetic() {
                line_num.push(part);
            }
        }

        let len = line_num.len();

        let mut num = String::new();

        if len ==  1{
            num.push_str(&line_num);
            num.push_str(&line_num);
        }else if len >= 2 {
            if let (Some(first), Some(last)) = (line_num.chars().next(), line_num.chars().last()) {
                num.push(first);
                num.push(last);
            } 
        }else if len == 0{
            num.push('0');
        }

        let line_result: Result<i32, _> = num.parse();
        println!("NUM: {}", num);


        match line_result {
            Ok(parsed_num) => {
                result += parsed_num as usize;
               
            }
            Err(err) => {
                println!("Error-Converting String to usize-int: {}", err);
             }
        }


        
       
    }

    println!("Teil-1 : {}", result);




}

// fn print_list(list: &Vec<String>){
//     for line in list {
//         println!("{}",line);
//     }

// }

fn  read_input() -> Vec<String> {
    let mut input = String::new();
    let mut run_input: bool  = true;
    let mut input_list: Vec<String> = Vec::new();

    while run_input == true {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim().to_string();
        match trimmed_input.as_str(){
             "stop" =>  run_input=false,
             _ => input_list.push(trimmed_input.clone()),
        }
        input.clear();
    }
  

    
    return input_list;
}

