use makepad_widgets::DefaultNone;
use moxin_protocol::data::FileID;

#[derive(Clone, DefaultNone, Debug)]
pub enum ChatAction {
    Start(FileID),
    Resume,
    None,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum DownloadAction {
    Play(FileID),
    Pause(FileID),
    Cancel(FileID),
    None,
}
