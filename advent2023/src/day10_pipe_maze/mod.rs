mod map;
mod path;
mod pipe_type;
mod position;
mod services;

pub fn entrypoint() -> Result<String, String> {
    services::part_1()
}
