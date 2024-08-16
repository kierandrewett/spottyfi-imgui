#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppCommand {
    About,
    Quit,

    ZoomIn,
    ZoomOut,
    ZoomReset,

    Navigate(&'static str),

    OpenSpotifyAccount,
}
