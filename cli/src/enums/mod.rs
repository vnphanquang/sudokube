#[derive(Debug)]
pub enum Navigation {
    Col(i8),
    Row(i8),
    Group(i8),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum RenderVariant {
    Default,
    Error, // for cells with same value but invalid position (same row / col / subgrid)
    Fixed, // cell not editable
    DirectionalRelative, // for cells in same col/row
    SameValue, // for cells with same value
}

impl std::fmt::Display for RenderVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
