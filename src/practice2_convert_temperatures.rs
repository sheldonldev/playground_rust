use assert_approx_eq::assert_approx_eq;

pub fn convert_fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
pub fn convert_celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

pub fn test_convert_temperatures() {
    let f: f64 = 0.0;
    let c: f64 = convert_fahrenheit_to_celsius(f);
    assert_approx_eq!(c, -17.8, 0.1);
    println!("{f} F degree equals to {:.2} C degree, pass!", c);

    let c: f64 = 5.0;
    let f: f64 = convert_celsius_to_fahrenheit(c);
    assert_approx_eq!(f, 41.0, 0.1);
    println!("{c} C degree equals to {:.2} F degree, pass!", f)
}