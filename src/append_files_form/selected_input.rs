#[derive(Clone, Default, PartialEq)]
pub enum SelectedInput {
    #[default]
    Links,
    AddLinks,
}

impl SelectedInput {
    const ORDER: &'static [SelectedInput] = &[
        SelectedInput::Links,
        SelectedInput::AddLinks,
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
