mod sequence;
pub fn entrypoint() -> Result<String, String> {
    println!("Day 9 Maintenance maintenance");
    // The input
    let input = include_str!("resources/input.txt");
    // Parse to create the Sequences
    let sequences: Vec<sequence::Sequence> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|err| format!("parse error {:?}", err).to_string())?;
    // Do the sum of the predictions
    let prediction_sum: i32 = sequences.iter().map(sequence::Sequence::predict_next).sum();
    // Do the sum of the extrapolations
    let extrapolation_sum: i32 = sequences
        .iter()
        .map(sequence::Sequence::extrapolate_previous)
        .sum();
    Ok(
        format!(
            "The result from OASIS is {} for the sum of predictions and {} for the sum of extrapolations",
            prediction_sum, extrapolation_sum
        ).to_string()
    )
}
