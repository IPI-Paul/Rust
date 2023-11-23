#![allow(unused)]
pub struct Vectors;
impl Vectors {
    pub fn exercise_001() {
        let arr: [u8; 3] = [1, 2, 3];
        let v = Vec::from(arr);
        is_vec(v);

        let v = vec![1, 2, 3];
        is_vec(v);

        let v = vec!(1, 2, 3);
        is_vec(v.clone());

        let v1 = Vec::from_iter(arr.iter().map(|x| *x as i32));
        is_vec(v1.clone());

        assert_eq!(v, v1);

        println!("Success!\narr: {:?}\nv: {:?}\nv1: {:?}", arr, v, v1);

        // Or

        let v: Vec<u8> = vec![1, 2, 3];
        is_vec(v);

        let v: Vec<u8> = vec!(1, 2, 3);
        is_vec(v.clone());

        let mut v1 = Vec::new();
        is_vec(v1.clone());

        for i in &v {
            v1.push(*i)
        }

        assert_eq!(v, v1);

        println!("Success!\narr: {:?}\nv: {:?}\nv1: {:?}", arr, v, v1);

        fn is_vec<T>(v: T) {
            
        }
    }
    pub fn exercise_002() {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);
        let mut v2 = Vec::new();
        // v2.extend(v1.iter());
        // Or
        v2.extend(&v1);
        assert_eq!(v1, v2);
        println!("Success!\nv1: {:?}\nv2: {:?}", v1, v2);
    }
    pub fn exercise_003() {
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.to_vec();
        // Or
        let v2: Vec<i32> = arr.into();
        assert_eq!(v1, v2);

        let s = "hello".to_string();
        let v1: Vec<u8> = s.into();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        let v4 : Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);
        println!(
            "Success!\narr: {:?}\nv1: {:?}\nv2: {:?}\nv3: {:?}\nv4: {:?}",
            arr, v1, v2, v3, v4
        );
    }
    pub fn exercise_004() {
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..3 {
            println!("{:?}", v[i])
        }
        for i in 0..5 {
            if i < v.len() {
                v[i] += 1
            } else {
                v.push(i + 2)
            }
        }
        // Or
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..5 {
            println!("{:?}", v.get(i))
        }
        for i in 0..5 {
            match v.get(i) {
                Some(e) => v[i] = e + 1,
                None => v.push(i + 2),
            }
        }
        assert_eq!(v, vec![2, 3, 4, 5, 6]);
        println!("Success!\nv: {:?}", v);
    }
    pub fn exercise_005() {
        let mut v = vec![1, 2, 3];
        let slice1 = &v[..];
        let slice2 = &v[..v.len()];
        assert_eq!(slice1, slice2);
        println!("v: {:?}\nslice1: {:?}\nslice2: {:?}", &v, slice1, slice2);

        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);
        let slice3 = &v[0..];
        // slice3.push(4) can't because slices are immutable.
        assert_eq!(slice3, &[1, 2, 3, 4]);
        println!("Success!\nslice3: {:?}", slice3);
    }
    pub fn exercise_006() {
        // Vectors only reallocate when the len() greater than capacity.
        let mut vec = Vec::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);
        for i in 0..10 {
            vec.push(i)
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);
        println!("vec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
        vec .push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);
        println!("vec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
        let mut vec = Vec::with_capacity(100);
        for i in 0..100 {
            vec.push(i)
        }
        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);
        println!("Success\nvec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
    }
    pub fn exercise_007() {
        #[derive(Debug, PartialEq)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let v: Vec<IpAddr> = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));
        println!("Success!\nv: {:?}", v);
    }
    pub fn exercise_008() {
        trait IpAddr {
            fn display(&self);
        }
        struct V4(String);
        impl IpAddr for V4 {
            fn display(&self) {
                println!("ipv4: {:?}", self.0)
            }
        }
        struct V6(String);
        impl IpAddr for V6 {
            fn display(&self) {
                println!("ipv6: {:?}", self.0)
            }
        }
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
        for ip in v {
            ip.display()
        }
    }
}
