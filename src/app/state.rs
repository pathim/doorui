#[derive(Debug, Default, Clone, Copy)]
pub enum Tabs {
    #[default]
    Edit = 0,
    Logs,
}

impl Tabs {
    pub fn names() -> Vec<&'static str> {
        vec!["Edit", "Logs"]
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub tab: Tabs,
}
