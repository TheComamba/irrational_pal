pub(crate) mod e;
pub(crate) mod extraction;
pub(crate) mod pi;

#[derive(Debug, Clone)]
pub(crate) struct Number {
    pub(crate) name: &'static str,
    pub(crate) digits: &'static str,
}

pub(crate) static NAN: Number = Number {
    name: "NaN",
    digits: "",
};

impl Default for &Number {
    fn default() -> Self {
        &NAN
    }
}
