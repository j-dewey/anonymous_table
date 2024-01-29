//
//  Testing anonymity
//

#[cfg(test)]
mod anonymous_test{
    use crate::anonymous::{self, Anonymous};

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct ComplexStruct{
        one: i32,
        two: ContainsPrimitiveValue
    }
    // this is just for testing purposes, in real use 0 is an invalid id
    impl anonymous::Anonymous for ComplexStruct{ fn id() -> u16 { 0 } }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Empty{
        One,
        Two
    }
    // this is just for testing purposes, in real use 0 is an invalid id
    impl anonymous::Anonymous for Empty{ fn id() -> u16 { 0 } }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum ContainsPrimitiveValue{
        One(i32),
        Two(i32)
    }
    // this is just for testing purposes, in real use 0 is an invalid id
    impl anonymous::Anonymous for ContainsPrimitiveValue{ fn id() -> u16{ 0 } }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum ContainsComplex{
        One(ComplexStruct),
        Two(ComplexStruct)
    }
    impl anonymous::Anonymous for ContainsComplex{ fn id() -> u16 { 0 } }

    //
    //  Assignments
    //
    #[test]
    fn assign_primitive() {
        let data = 120;
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(data, *unsafe{ cell.get_ref()} );
    }
    #[test]
    fn assign_complex() {
        let data = ComplexStruct{
            one: 1,
            two: ContainsPrimitiveValue::Two(2)
        };
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(data, *unsafe{ cell.get_ref()} );
    }
    #[test]
    fn assign_none_primitive() {
        let data: Option<i32> = None;
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(None, *unsafe{ cell.get_ref::<Option<i32>>()} );
    }
    #[test]
    fn assign_some_primitive() {
        let data: Option<i32> = Some(1);
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(Some(1), *unsafe{ cell.get_ref::<Option<i32>>()} );
    }
    #[test]
    fn assign_empty_enum() {
        let data = Empty::One;
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(Empty::One, *unsafe{ cell.get_ref::<Empty>()} );
        assert_ne!(Empty::Two, *unsafe{ cell.get_ref::<Empty>()} );
    }
    #[test]
    fn assign_primitive_enum() {
        let data = ContainsPrimitiveValue::One(1);
        let cell = anonymous::AnonymousCell::new(data);
        assert_eq!(ContainsPrimitiveValue::One(1), *unsafe{ cell.get_ref::<ContainsPrimitiveValue>()} );
        assert_ne!(ContainsPrimitiveValue::One(2), *unsafe{ cell.get_ref::<ContainsPrimitiveValue>()} );
        assert_ne!(ContainsPrimitiveValue::Two(1), *unsafe{ cell.get_ref::<ContainsPrimitiveValue>()} );
        assert_ne!(ContainsPrimitiveValue::Two(2), *unsafe{ cell.get_ref::<ContainsPrimitiveValue>()} );
    }
    #[test]
    fn assign_complex_enum() {
        let data = ComplexStruct{ one: 1, two: ContainsPrimitiveValue::Two(2) };
        let alternative = ComplexStruct{ one: 2, two: ContainsPrimitiveValue::One(1) };
        let cell = anonymous::AnonymousCell::new(ContainsComplex::One(data));
        assert_eq!(ContainsComplex::One(data), *unsafe{ cell.get_ref() });
        assert_ne!(ContainsComplex::One(alternative), *unsafe{ cell.get_ref() });
        assert_ne!(ContainsComplex::Two(data), *unsafe { cell.get_ref() });
        assert_ne!(ContainsComplex::Two(alternative), *unsafe{ cell.get_ref() });
    }

    //
    //  Updates
    //
    #[test]
    fn reassign() {
        let data = 10;
        let mut cell = anonymous::AnonymousCell::new(data);
        unsafe{
            let old = cell.exchange_value(2);
            assert_eq!(10, old);
        }
        assert_eq!(2, *unsafe{ cell.get_ref() });
    }
    #[test]
    fn update_inplace() {
        let data = ComplexStruct{ one: 1, two: ContainsPrimitiveValue::Two(2) };
        let expected = ComplexStruct{ one: 2, two: ContainsPrimitiveValue::Two(2) };
        let mut cell = anonymous::AnonymousCell::new(data);
        unsafe{
            let ptr = cell.get_mut::<ComplexStruct>();
            ptr.one = 2;
            assert_eq!(*cell.get_ref::<ComplexStruct>(), expected);
            assert_ne!(*cell.get_ref::<ComplexStruct>(), data);
        }
    }
    #[test]
    fn multiple_updates() {
        let initial = ComplexStruct{ one: 1, two: ContainsPrimitiveValue::Two(2) };
        let second = ComplexStruct{ one: 2, two: ContainsPrimitiveValue::Two(2) };
        let third = ComplexStruct{ one: 2, two: ContainsPrimitiveValue::One(1) };
        let mut cell = anonymous::AnonymousCell::new(initial);
        unsafe{
            let ptr = cell.get_mut::<ComplexStruct>();
            assert_eq!(*ptr, initial);
            ptr.one = 2;
            assert_eq!(*ptr, second);
            assert_ne!(*ptr, initial);
            ptr.two = ContainsPrimitiveValue::One(1);
            assert_eq!(*ptr, third);
            assert_ne!(*ptr, second);
        }
    }
    #[test]
    fn evil_things() {
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        struct Breakable{ one: i32, two: i32 }
        impl anonymous::Anonymous for Breakable{ fn id() -> u16 { 0 } }

        let data = Breakable{ one: 1, two: 2};
        let cell = anonymous::AnonymousCell::new(data);
        unsafe{
            let ptr = cell.get_ref::<i64>();
            let nums: [i32; 2] = std::mem::transmute(*ptr);
            assert_eq!(nums, [1, 2]);
        }
    }
    #[test]
    fn evil_bad_things() {
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
    #[test]
    fn edit_under_scope() {
        let data = 1;
        let mut cell = anonymous::AnonymousCell::new(data);
        unsafe {
            cell.exchange_value(2);
        }
        assert_eq!(*unsafe{ cell.get_ref::<i32>() }, 2);
    }
    #[test]
    fn edit_with_move() {
        let data = 1;
        let mut cell = anonymous::AnonymousCell::new(data);
        let mut change = || { 
            unsafe{ cell.exchange_value(2); }
        };
        change();
        assert_eq!(*unsafe{ cell.get_ref::<i32>() }, 2)
    }
}

//
//  Testing rows
//

#[cfg(test)]
mod row_test{
    use crate::table::AnonymousRow;
    use crate::anonymous::Anonymous;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct ComplexStruct{
        one: i32,
        two: u32
    }
    // this is just for testing purposes, in real use 0 is an invalid id
    impl Anonymous for ComplexStruct{ fn id() -> u16 { 0 } }

    #[test]
    fn push() {
        let mut row = AnonymousRow::new();
        assert_eq!(row.len(), 0);
        row.push(1);
        assert_eq!(row.len(), 1);
        row.push(2.0);
        assert_eq!(row.len(), 2);
        row.push(3u32);
        assert_eq!(row.len(), 3);
    }
    #[test]
    fn get() {
        let mut row = AnonymousRow::new();
        row.push(1);
        row.push(2.0);
        row.push(3u32);

        assert_eq!(1, *unsafe{ row.get_at(0).unwrap_unchecked() });
        assert_eq!(2.0, *unsafe{ row.get_at(1).unwrap_unchecked() });
        assert_eq!(3u32, *unsafe{ row.get_at(2).unwrap_unchecked() });
    }
    #[test]
    fn change() {
        let mut row = AnonymousRow::new();
        row.push(1);
        row.push(2.0);
        row.exchange_at(3, 0).unwrap();
        assert_eq!(3, *unsafe{ row.get_at(0).unwrap_unchecked() } );
        assert_eq!(2.0, *unsafe{ row.get_at(1).unwrap_unchecked() });
    }
    #[test]
    fn update() {
        let mut row = AnonymousRow::new();
        row.push(ComplexStruct{ one: 1, two: 2 });
        row.push(2.0);
        unsafe{
            let ptr: &mut ComplexStruct = row.get_mut_at(0).unwrap_unchecked();
            ptr.one = 2;
            assert_eq!(*row.get_at::<ComplexStruct>(0).unwrap_unchecked(), ComplexStruct{ one: 2, two: 2});
        }
    }
}