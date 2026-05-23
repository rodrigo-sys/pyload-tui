pub enum AppAction {
    Quit,
    GoToPackages,
    GoToFiles(i32, String),
    GoToPreviousScreen,
    OpenAddPackageForm,
    OpenAppendFilesForm(i32, String),
    DeletePackages(Vec<i32>),
    DeleteFiles(Vec<i32>),
}
