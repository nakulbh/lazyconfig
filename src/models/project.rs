#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub agent_platforms: Vec<String>,
}
