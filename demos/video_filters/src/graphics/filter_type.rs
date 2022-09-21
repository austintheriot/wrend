use std::iter::Filter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Grayscale,
}

impl FilterType {
    pub fn plain_text_label(&self) -> String {
        match self {
            FilterType::Unfiltered => String::from("Unfiltered"),
            FilterType::Grayscale => String::from("Grayscale"),
        }
    }
}