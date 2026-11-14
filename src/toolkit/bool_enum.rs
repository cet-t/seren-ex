#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Bool {
    #[name_localized("ja", "はい")]
    True,
    #[name_localized("ja", "いいえ")]
    False,
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        match value {
            Bool::True => true,
            Bool::False => false,
        }
    }
}
