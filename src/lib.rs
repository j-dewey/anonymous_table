mod anonymous;
pub use anonymous::Anonymous;
mod table;
pub use table::{
    AnonymousTable,
    AnonymousRow,
    TableIndex,
    RowName
};