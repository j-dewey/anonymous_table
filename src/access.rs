// 
// These are accessors used to index into the table
//

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TableIndex{pub(crate) row: usize, pub(crate) column: usize}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct RowName(pub u8); // 255 named rows 
