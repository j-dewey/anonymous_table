mod anonymous;
mod table;
mod tests;

fn main(){
    use anonymous::Anonymous;
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct Larger([i32; 10]);
    impl Anonymous for Larger{ fn id() -> u16 { 0 }}

    let data = 1;
    let cell = anonymous::AnonymousCell::new(data);
    unsafe{
        // larger than the data allocated
        // should panic
        let ptr = cell.get_ref::<Larger>();
        println!("{:?}", *ptr);
    }
}