pub enum AppAction {
    Quit,
    GoToPackages,
    GoToFiles(i32),
    GoToPreviousScreen,
    OpenAddPackageForm,
    OpenAppendFilesForm(i32, String),
    DeletePackages(Vec<(usize, i32)>),
    DeleteFiles(Vec<(usize, i32)>),
}
