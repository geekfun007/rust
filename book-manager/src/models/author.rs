#[derive(Debug, Clone)]
pub struct Author {
    pub id: u32,
    pub name: String,
}

impl Author {
    pub fn new(id: u32, name: String) -> Self {
        assert!(!name.is_empty(), "作者名不能为空");
        Self { id, name }
    }
}
