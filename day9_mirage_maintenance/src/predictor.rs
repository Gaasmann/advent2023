pub struct Predictor {
    data: Vec<i32>
}

impl Predictor {
    pub fn new(data: Vec<i32>) -> Self {
        Predictor{data}
    }
}


#[cfg(test)]
