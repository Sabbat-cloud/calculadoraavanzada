use super::Calculator;

fn approx(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn scientific_and_invalid_numbers() {
    let mut c = Calculator::new();
    assert!(approx(c.evaluate("1e6").unwrap(), 1_000_000.0, 1e-6));
    assert!(c.evaluate("1e").is_err());
    assert!(c.evaluate(".").is_err());
}

#[test]
fn atan2_hypot_root() {
    let mut c = Calculator::new(); // DEG
    assert!(approx(c.evaluate("hypot(3,4)").unwrap(), 5.0, 1e-12));
    assert!(approx(c.evaluate("atan2(1,1)").unwrap(), 45.0, 1e-9));
    assert!(approx(c.evaluate("root(3,8)").unwrap(), 2.0, 1e-9));
}

#[test]
fn rounding_and_units() {
    let mut c = Calculator::new();
    assert!(approx(c.evaluate("round(3.7)").unwrap(), 4.0, 1e-12));
    assert!(approx(c.evaluate("trunc(-3.7)").unwrap(), -3.0, 1e-12));
    assert!(approx(c.evaluate("sign(-5)").unwrap(), -1.0, 1e-12));
    assert!(approx(c.evaluate("rad2deg(pi)").unwrap(), 180.0, 1e-9));
}

#[test]
fn combinatorics() {
    let mut c = Calculator::new();
    assert!(approx(c.evaluate("fact(5)").unwrap(), 120.0, 1e-12));
    assert!(approx(c.evaluate("comb(5,2)").unwrap(), 10.0, 1e-12));
    assert!(approx(c.evaluate("perm(5,2)").unwrap(), 20.0, 1e-12));
}

