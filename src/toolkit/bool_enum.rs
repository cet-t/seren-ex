#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Bool {
    #[name_localized("ja", "はい")]
    True,
    #[name_localized("ja", "いいえ")]
    False,
}
