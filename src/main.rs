
mod  day_2;
mod day_1;
mod input_reader;
mod day_3;

// use day_1::day_1 as d1;
// use day_2::day_2 as d2;
// use day_3::day_3 as d3;




fn main() {
    //Day 1
    // d1::solution_1();

    //Day 2
    // d2::solution_2();

    //Day3
    // d3::solution_3();



    //binarySearch
    let mut input_list: Vec<i32> = Vec::new();
    input_list.push(1);
    input_list.push(2);
    input_list.push(3);
    input_list.push(10);
    input_list.push(30);
    input_list.push(34);
    let search : i8  = 34;

    let i = binary_search(input_list, search);
    println!("Index: {}", i  );


   let mut arr = [123,234 ,1 , 2, 5 ,6 ,7,8];

   let arr =  bubble_sort(&mut arr);

    //bubbleSort print
    for i in 0..arr.len(){
        println!("{}", arr[i]);

    }





}


fn  bubble_sort(arr: &mut[i32]) -> &mut[i32]{
    let  length  = arr.len();

    let mut temp : i32 = 0;

    for i in 0..length-1 {

        for j in 1..length-i {

            if(arr[j-1] < arr[j]){
                //swap
                temp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j]  =  temp;

            }
        }

    }

    return arr;


}

fn binary_search(list: Vec<i32>, target_val: i8) -> i8{
    let mut low: i8 = 0;
    let mut high: i8 =  list.len() as i8 - 1;

    let mut mid_index = 0 ;


    while (low <= high) {
        mid_index = ((high - low) / 2) + low;
        let val: i8 = list[mid_index as usize] as i8;

        if (val == target_val) {
            return mid_index;
        }
        if (val > target_val) {
            high = mid_index - 1;
            continue;
        }
        if (val < target_val) {
            low = mid_index + 1;
            continue;
        }
    }
    return -1;





}