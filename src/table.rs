use std::collections::HashMap;

use crate::{anonymous::{Anonymous, AnonymousCell}, RowName, TableIndex, Tag};

///
/// An [AnonymousRow] represents a row in an [AnonymousTable]. It is considered anonymous
/// since the compiler loses all type information on an object added to it. 
/// 
#[derive(Debug)]
pub struct AnonymousRow(Vec<AnonymousCell>);

impl AnonymousRow{
    pub fn new() -> Self{ Self(Vec::new()) }
    pub fn with_capacity(cap: usize) -> Self{ Self(Vec::with_capacity(cap)) }
    pub fn len(&self) -> usize{ self.0.len() }

    pub fn push<T: Anonymous>(&mut self, item: T){ self.0.push(AnonymousCell::new(item)) }
    pub fn chain_push<T: Anonymous>(mut self, item: T) -> Self{
        self.push(item);
        self
    }
    pub fn insert<T: Anonymous>(&mut self, item: T, index: usize){ 
        self.0.insert(
            index, 
            AnonymousCell::new(item)
        ) 
    }
    pub fn exchange_at<T: Anonymous + Copy>(&mut self, item: T, index: usize) -> Result<(), ()>{
        let cell = match self.0.get_mut(index){
            Some(cell) => cell,
            None => return Err(())
        };
        if T::id() != cell.id{ return Err(()) }
        unsafe{ cell.exchange_value(item) };
        Ok(())
    }

    pub fn get_at<T: Anonymous>(&self, index: usize) -> Option<&T>{
        let cell = self.0.get(index)?;
        if cell.id != T::id(){ return None; }
        Some( unsafe{ cell.get_ref() } )
    }
    pub fn get_mut_at<T: Anonymous>(&mut self, index: usize) -> Option<&mut T>{
        let cell = self.0.get_mut(index)?;
        if cell.id != T::id(){ return None; }
        Some( unsafe { cell.get_mut() } )
    }
    pub unsafe fn get_at_unchecked<T: Anonymous>(&self, index: usize) -> &T{
        self.0[index].get_ref()
    }
    pub unsafe fn get_at_mut_unchecked<T: Anonymous>(&mut self, index: usize) -> &mut T{
        self.0[index].get_mut()
    }
    // There is no removal since that would mess up table indices
    // priavte methods:
    fn get_ids(&self) -> Vec<u16>{
        self.0.iter()
        .map(|anon| anon.id)
        .collect()
    }
}

#[derive(Debug)]
struct TableRaw{
    rows: Vec<AnonymousRow>
}

impl TableRaw{
    fn get_row(&self, row: usize) -> Option<&AnonymousRow>{
        self.rows.get(row)
    }

    fn get_row_mut(&mut self, row: usize) -> Option<&mut AnonymousRow>{
        self.rows.get_mut(row)
    }
}

// Table isn't square and the columns / rows don't mean anything special.
// A cell can hold any sized data of any type. 
// the only guarantee is that cells in reserved columns will hold a value
// of a specific type. When a reservation is requested, the first available
// column will be reserved

#[derive(Debug)]
pub struct AnonymousTable{
    table: TableRaw,
    cell_locations: HashMap<u16, Vec<TableIndex>>,
    // the location in this Vec represents column in table
    reserved: Vec<usize>,
    names: HashMap<RowName, usize>,
    tags: HashMap<Tag, Vec<usize>>
}

impl AnonymousTable{
    pub fn new() -> Self{
        Self { 
            table: TableRaw { 
                rows: Vec::new() 
            },
            cell_locations: HashMap::new(),
            reserved: Vec::new(),
            names: HashMap::new(),
            tags: HashMap::new()
        }
    }

    pub fn clear(&mut self){
        self.table.rows.clear();
        self.cell_locations.clear();
        self.names.clear();
    }

    pub fn len(&self) -> usize{
        self.table.rows.len()
    }

    pub fn capacity(&self) -> usize{
        self.table.rows.capacity()
    }

    pub fn reserve(&mut self, increase: usize){
        self.table.rows.reserve(increase)
    }

    // 
    //  Adding
    // 

    fn register_row(&mut self, ids: Vec<u16>){
        let row_id = self.table.rows.len();
        for (col, id) in ids.iter().enumerate(){
            let index = TableIndex{ row: row_id, column: col };
            match self.cell_locations.get_mut(&id){
                Some(vec) => { vec.push(index) },
                None => {
                    let vec = vec![index];
                    self.cell_locations.insert(*id, vec);
                }
            }
        }
    }

    pub fn register_named_row(&mut self, row: AnonymousRow, name: RowName){
        let index = self.table.rows.len();
        self.register_row(row.get_ids());
        self.table.rows.push(row);
        self.names.insert(name, index);
    }

    pub fn push_row(&mut self, row: AnonymousRow){
        self.register_row(row.get_ids());
        self.table.rows.push(row)
    }

    pub fn chain_push_row(mut self, row: AnonymousRow) -> Self{
        self.push_row(row);
        self
    }

    pub fn insert_at<T: Anonymous>(&mut self, obj: T, location: TableIndex){
        self.table
            .get_row_mut(location.row)
            // fix this
            .expect("Indexed of insert beyond made rows")
            .0
            .insert(location.column, AnonymousCell::new(obj))
    }

    //
    //  Getting
    //

    // This gets the row based off its index in [TableRaw],
    // not based on its name
    pub fn get_row(&self, row: usize) -> Option<&AnonymousRow>{
        self.table.get_row(row)
    } 

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut AnonymousRow>{
        self.table.get_row_mut(row)
    }

    pub fn get_named_row(&self, name: RowName) -> Option<&AnonymousRow>{
        let index = self.names.get(&name)?;
        self.table.get_row(*index)
    }

    pub fn get_named_row_mut(&mut self, name: RowName) -> Option<&mut AnonymousRow>{
        let index = self.names.get(&name)?;
        self.table.get_row_mut(*index)
    }

    pub fn get_all_of_type<T: Anonymous>(&self) -> Vec<&T>{
        let id = T::id();
        let locs = match self.cell_locations.get(&id){
            Some(vec) => { vec } ,
            None => { return Vec::new() } 
        };
        let mut vals = Vec::with_capacity(locs.len());
        for loc in locs{
            let data = self.table.rows[loc.row].get_at::<T>(loc.column).unwrap();
            vals.push(data);
        }
        vals
    }

    //
    // Using [Tag]s
    //

    pub fn register_tagged_row(&mut self, row: AnonymousRow, tag: Tag){
        let index = self.table.rows.len();
        self.register_row(row.get_ids());
        self.table.rows.push(row);
        match self.tags.get_mut(&tag){
            Some(vec) => vec.push(index),
            None => { self.tags.insert(tag, vec![index]); }
        }
    }

    pub fn registed_named_tagged_row(&mut self, row: AnonymousRow, tag: Tag, name: RowName){
        let index = self.table.rows.len();
        self.register_row(row.get_ids());
        self.table.rows.push(row);
        self.names.insert(name, index);
        match self.tags.get_mut(&tag){
            Some(vec) => vec.push(index),
            None => { self.tags.insert(tag, vec![index]); }
        }
    }

    pub fn get_tagged_rows(&self, tag: Tag) -> Option<Vec<&AnonymousRow>>{
        let row_inds = self.tags.get(&tag)?;
        let mut rows = Vec::with_capacity(row_inds.len());
        for ind in row_inds{
            rows.push(
                self.table.get_row(*ind).expect("Tagged row location moved")
            );
        }
        Some(rows)
    }
}