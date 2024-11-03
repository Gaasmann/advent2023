use crate::day10_pipe_maze::map::Map;

pub fn part_1() -> Result<String, String> {
    let data = include_str!("resources/input.txt");
    let map: Map = data.parse().unwrap();
    let path = crate::day10_pipe_maze::path::Path::new(&map);
    Ok(format!(
        "The number of steps needed to be the furthest: {}",
        path.steps_to_be_farthest()
    ))
}
