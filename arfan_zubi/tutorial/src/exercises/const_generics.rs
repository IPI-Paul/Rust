#![allow(unused)]
pub struct ConstGenerics;
impl ConstGenerics {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Array<T, const N: usize> {
            data: [T; N],
        }
        let arrays: [Array<i32, 3>; 3]  = [
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [4, 5, 6]
            }
        ];
        let floats: [Array<f64, 2>; 3]  = [
            Array {
                data: [1.0, 2.0],
            },
            Array {
                data: [3.0, 4.0],
            },
            Array {
                data: [5.0, 6.0]
            }
        ];
        println!("Success!\narrays: {:?}\nfloats: {:?}", arrays, floats);
    }
}
