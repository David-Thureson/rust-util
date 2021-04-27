pub const ML_PER_FL_OZ: f64 = 29.5735295625;
pub const GRAMS_PER_OZ: f64 = 28.34952;

pub fn ml_to_fl_oz<T>(val: T) -> f64
    where T: Into<f64>
{
    val.into() / ML_PER_FL_OZ
}

pub fn fl_oz_to_ml<T>(val: T) -> f64
    where T: Into<f64>
{
    val.into() * ML_PER_FL_OZ
}

pub fn grams_to_oz<T>(val: T) -> f64
    where T: Into<f64>
{
    val.into() / GRAMS_PER_OZ
}

pub fn oz_to_grams<T>(val: T) -> f64
    where T: Into<f64>
{
    val.into() * GRAMS_PER_OZ
}




