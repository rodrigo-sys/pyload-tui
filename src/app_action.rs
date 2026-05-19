pub enum AppAction {
    Quit,
    GoToPackages,
    GoToFiles(i32),
    OpenAddPackageForm,
    OpenAppendFilesForm(i32, String),
    DeletePackages(Vec<(usize, i32)>),
}
