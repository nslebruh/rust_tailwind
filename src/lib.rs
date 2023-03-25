pub const Zero: &'static str = "0";
pub const ZeroPointFive: &'static str = "0.125";
pub const One: &'static str = "0.25";
pub const OnePointFive: &'static str = "0.375";
pub const Two: &'static str = "0.5";
pub const TwoPointFive: &'static str = "0.625";
pub const Three: &'static str = "0.75";
pub const ThreePointFive: &'static str = "0.875";
pub const Four: &'static str = "1.0";

pub struct Tailwind {
    pub classes: Vec<Box<dyn CSSClass>>
}

impl Tailwind {
    pub fn to_string_vec(&self) -> Vec<String> {
        self.classes.iter().map(|x| x.to_string()).collect()
    }
}

impl From<Box<dyn CSSClass>> for String {
    fn from(value: Box<dyn CSSClass>) -> Self {
        value.to_string()
    }
}
pub trait CSSClass {
    fn to_string(&self) -> String;
}

pub struct Width;

pub struct Height;

macro_rules! impl_1 {
    ($t:tt) => {
        #[allow(non_upper_case_globals)]
        impl $t {
            pub const Zero: &'static str = "0";
            pub const ZeroDotFive: &'static str = "0.125";
            pub const One: &'static str = "0.25";
            pub const OneDotFive: &'static str = "0.375";
            pub const Two: &'static str = "0.5";
            pub const TwoPointFive: &'static str = "0.625";
            pub const Three: &'static str = "0.75";
            pub const ThreeDotFive: &'static str = "0.875";
            pub const Four: &'static str = "1.0";
        }
    };
}

impl_1!(Width);
impl_1!(Height);

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
