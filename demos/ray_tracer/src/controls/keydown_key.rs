pub enum KeydownKey {
    W,
    A,
    S,
    D,
    Space,
    Shift,
}

impl TryFrom<String> for KeydownKey {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "w" | "W" => Ok(Self::W),
            "a" | "A" => Ok(Self::A),
            "s" | "S" => Ok(Self::S),
            "d" | "D" => Ok(Self::D),
            " " => Ok(Self::Space),
            "Shift" => Ok(Self::Shift),
            _ => Err(()),
        }
    }
}
