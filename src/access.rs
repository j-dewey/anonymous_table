// 
// These are accessors used to index into the table
//

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TableIndex{pub(crate) row: usize, pub(crate) column: usize}

// A [RowName] links to a specific row
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct RowName(pub u8); // 255 named rows 

// A [Tag] links to multiple rows
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Tag(pub(crate) u8); // 255 tagged rows