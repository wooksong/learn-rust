pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let converted = match (from_unit, to_unit) {
        ("C", "C") | ("F", "F") | ("K", "K") => value,
        ("F", "C") => (value - 32.) * (5. / 9.),
        ("F", "K") => (value - 32.) * (5. / 9.) + 273.15,
        ("C", "F") => value * (9. / 5.) + 32.,
        ("C", "K") => value + 273.15,
        ("K", "C") => value - 273.15,
        ("K", "F") => (value - 273.15) * (9. / 5.) + 32.,
        _ => return Err("Invalid unit".into()),
    };
    Ok(converted)
}
