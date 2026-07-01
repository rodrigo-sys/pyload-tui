use openapi::models::Destination;

pub enum AppAction {
    Quit,
    GoToPackages,
    GoToFiles(i32, String),
    GoToPreviousScreen,
    OpenAddPackageForm,
    OpenAppendFilesForm(i32, String),
    DeletePackages(Vec<i32>),
    DeleteFiles(Vec<i32>),
    StopDownloads(Vec<i32>),
    RestartFile(i32),
    RestartPackage(i32),
    RestartFailed,
    ReorderPackage(i32, i32),
    MovePackage(Destination, i32),
}
