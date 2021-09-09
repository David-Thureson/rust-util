#[inline]
pub fn discriminant(a: f64, b: f64, c: f64) -> f64 {
    (b * b) - (4.0 * a * c)
}


#[inline]
pub fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    (quadratic_root(a, b, c, -1.0), quadratic_root(a, b, c, 1.0))
}

#[inline]
pub fn quadratic_root(a: f64, b: f64, c: f64, mult: f64) -> f64 {
    let num = (-1.0 * b) + (mult * discriminant(a, b, c).sqrt());
    let denom = 2.0 * a;
    num / denom
}
