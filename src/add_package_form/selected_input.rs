#[derive(Clone, Default, PartialEq)]
pub enum SelectedInput {
    #[default]
    Name,
    Links,
    Password,
    Queue,
    Collector,
    AddPackage,
}

impl SelectedInput {
    const ORDER: &'static [SelectedInput] = &[
        SelectedInput::Name,
        SelectedInput::Links,
        SelectedInput::Password,
        SelectedInput::Queue,
        SelectedInput::Collector,
        SelectedInput::AddPackage,
    ];

    pub fn next(&self) -> Self {
        self.cycle(1)
    }

    pub fn prev(&self) -> Self {
        self.cycle(Self::ORDER.len() - 1)
    }

    fn cycle(&self, step: usize) -> Self {
        let pos = Self::ORDER.iter().position(|s| s == self).unwrap_or(0);
        Self::ORDER[(pos + step) % Self::ORDER.len()].clone()
    }
}
