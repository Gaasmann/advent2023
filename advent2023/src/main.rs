use advent2023::day10_pipe_maze;
use advent2023::day9_mirage_maintenance;
fn main() {
    println!("Hello, world!");
    let entrypoints = vec![
        day9_mirage_maintenance::entrypoint,
        day10_pipe_maze::entrypoint,
    ];

    for entrypoint in entrypoints {
        let result = entrypoint();
        match result {
            Ok(str) => println!("SUCCESS: {}", str),
            Err(error) => println!("FAILURE: {}", error),
        }
    }
}
