use super::preferences::WidgetStatePreferences;

#[derive(Debug, Default)]
pub struct WidgetStateSearchPane {
    pub visible: bool,
    pub search_value: String,
}

#[derive(Debug, Default)]
pub struct WidgetStatePanes {
    pub home_visible: bool,

    pub preferences: WidgetStatePreferences,
    pub search: WidgetStateSearchPane,
}
