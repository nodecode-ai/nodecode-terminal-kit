/// Effect command produced by `Model::init` and `Model::update`.
pub enum Command<Msg> {
    None,
    One(Box<dyn FnOnce() -> Msg + Send>),
    Batch(Vec<Box<dyn FnOnce() -> Msg + Send>>),
}

impl<Msg> Command<Msg> {
    #[must_use]
    pub fn none() -> Self {
        Self::None
    }

    #[must_use]
    pub fn one<F>(f: F) -> Self
    where
        F: FnOnce() -> Msg + Send + 'static,
    {
        Self::One(Box::new(f))
    }

    #[must_use]
    pub fn batch<I, F>(commands: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() -> Msg + Send + 'static,
    {
        Self::Batch(
            commands
                .into_iter()
                .map(|f| Box::new(f) as Box<dyn FnOnce() -> Msg + Send>)
                .collect(),
        )
    }
}

impl<Msg> Default for Command<Msg> {
    fn default() -> Self {
        Self::None
    }
}
