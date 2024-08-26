#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    About,
    Quit,

    ZoomIn,
    ZoomOut,
    ZoomReset,

    Navigate(&'static str),

    DoSearch(String),

    OpenSpotifyAccount,
}
