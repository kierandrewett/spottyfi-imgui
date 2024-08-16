#[derive(Debug, Clone, Default)]
pub struct WidgetStateSearchPane {
    pub visible: bool,
    pub search_value: String,
}

#[derive(Debug, Clone, Default)]
pub struct WidgetStatePanes {
    pub preferences_visible: bool,
    pub home_visible: bool,

    pub search: WidgetStateSearchPane,
}
