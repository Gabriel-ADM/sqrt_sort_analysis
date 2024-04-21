mod algorithms;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use rand::Rng;
use std::time::{ Duration, Instant };
use std::io::BufReader;
use algorithms::*;

pub fn generate_data(data_size: &Vec<i32>) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\data_ten_to_{}.txt", index + 4);
        let mut file: File = File::create(&file_name).expect("Failed to create file");

        for _ in 0..n {
            let value = rand::thread_rng().gen_range(0..n);
            file.write_all(&format!("{}, ", value).as_bytes()).expect("Failed to write to file");
        }
    }
}

pub fn read_data(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Failed to get file");
    let buf_reader = BufReader::new(file);
    let mut data: Vec<i32> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("Error getting line");
        data = line
            .split(", ")
            .filter_map(|num_str| num_str.parse::<i32>().ok())
            .collect();
    }

    data
}

pub fn measure_execution_time<F, R>(closure: F) -> Duration where F: FnOnce() -> R {
    let start_time = Instant::now();
    let result: R = closure();
    let _aux: R = result;
    let duration: Duration = start_time.elapsed();

    duration
}

pub fn sort_and_measure(n: Vec<i32>, qt_avg: i32) {
    for i in 0..n.len() {
        let data_file_name = format!(r"src\data\data_ten_to_{}.txt", i + 4);
        let result_file_name = format!(r"src\result\execs_avg_ten_to_{}.csv", i + 4);
        let mut result_file = File::create(result_file_name).expect("Error creating file");

        let data = read_data(&data_file_name);
        for _i in 0..qt_avg {
            let result_string = &format!(
                "bubble_sort;{:?};heap_sort;{:?};\n",
                measure_execution_time(|| sqrt_sort(&data, "bubble".to_string())),
                measure_execution_time(|| sqrt_sort(&data, "heap".to_string()))
            );
            println!("{:?}", result_string);
            result_file.write_all(result_string.as_bytes()).expect(&"Failed to write file");
        }
    }
}

fn main() {
    let n = vec![
        10000,
         100000,
        1000000,
        10000000,
        100000000
    ];
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "generate-data" {
        println!("Generating data...");
        generate_data(&n);
    }
    sort_and_measure(n, 100)
}
