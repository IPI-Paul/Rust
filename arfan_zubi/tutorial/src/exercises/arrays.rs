pub struct Arrays;
impl Arrays {
    // Cannot be dynamical generated as the compiler needs to know the size at compile time.
    pub fn exercise_001() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert!(arr.len() == 5);
        println!("Success!\narr: {:?}", arr);
    }
    pub fn exercise_002() {
        // Let compiler infer both type and size.
        let arr0 = [1, 2, 3];
        // Set the size but leave the compiler to infer the type.
        let arr: [_; 3] = ['a', 'b', 'c'];
        assert!(std::mem::size_of_val(&arr) == 12);
        println!(
            "Success!\narr0: {:?}\narr: {:?} is {} bytes", arr0, arr, std::mem::size_of_val(&arr)
        );
    }
    pub fn exercise_003() {
        let list: [i32; 100] = [1; 100];
        assert!(list[0] == 1);
        assert!(list.len() == 100);
        println!("Success!\nlist has {} {}s", list.len(), list[0]);
    }
    pub fn exercise_004() {
        let _arr = [1, 2, 3];
        println!("Success!\n_arr: {:?}", _arr);
    }
    pub fn exercise_005() {
        let arr = ['a', 'b', 'c'];
        let ele = arr[0];
        assert!(ele == 'a');
        println!("Success!\narr: {:?}\nele: {}", arr, ele);
    }
    pub fn exercise_006() {
        let names = [String::from("Sunfei"), "Sunface".to_string()];
        // Get returns an Option<T> an is safe to use.
        let name0 = names.get(0).unwrap();
        // Indexing is not safe.
        // let name1 = &names[2];
        let name1 = names.get(2);
        println!("Success!\nnames: {:?}\nname0: {}\n_name1: {:?}", names, name0, name1);
    }
}
