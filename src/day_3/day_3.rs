use crate::input_reader::input_reader as reader;



// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

pub fn solution_3 () {
    let lines = reader::read_input();


    for line_num in 0..lines.len(){
        let line_top;
        let line = lines.get(line_num);
        let line_bottom;

        if(line_num != 0){
            line_top = lines.get(line_num-1);
        }
        if(line_num < lines.len()-1){
            line_bottom = lines.get(line_num+1)

        }

        let result = line.unwrap()
            .replace(".", "")
            .split_ascii_whitespace()
            .map(|col| col.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err));

        for r in result{
            println!("{}", r);
        }







    }

}