pub struct Path {
    pub components: alloc::vec::Vec<alloc::string::String>,
    pub absolute: bool,
}

impl Path {
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        let absolute = s.starts_with('/');
        let components: alloc::vec::Vec<alloc::string::String> = s
            .split('/')
            .filter(|c| !c.is_empty() && *c != ".")
            .map(|c| alloc::string::String::from(c))
            .collect();
        Path { components, absolute }
    }
}
