mod map;
mod path;
mod pipe_type;
mod position;
mod services;

pub fn entrypoint() -> Result<String, String> {
    let res_1 = services::part_1();
    let res_2 = services::part_2();
    match (res_1, res_2) {
        (Ok(x1), Ok(x2)) => Ok(format!("{} {}", x1, x2)),
        (Ok(x1), Err(x2)) => Err(format!("PART 1 OK: {} ERROR ON PART 2: {}", x1, x2)),
        (Err(x1), Ok(x2)) => Err(format!("ERROR ON PART 1: {} PART 2 OK: {}", x1, x2)),
        (Err(x1), Err(x2)) => Err(format!("ERROR ON ALL PARTS: {} {}", x1, x2)),
    }
}
