#[derive(Debug, Clone, Copy)]
pub enum Layout {
    //Match parent with min and max padding.
    MatchParent(u32, u32),
    ///Absolute layout implies that this layout is ignored.
    Absolute
}