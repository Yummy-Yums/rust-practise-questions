pub mod chp7 {

    macro_rules! create_function {
        ($func_name:ident) => {

            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }

        };
    }

    create_function!(foo);
    create_function!(bar);

    macro_rules! print_result {
        ($expression:expr) => {
            println!("{:?} = {:?}",
            stringify!($expression),
            $expression)
        };
    }

    macro_rules! addition {
        ($($num:expr),* $(,)?) => {

            {
                let mut result = 0;
                $(
                    result += $num as i32;
                )*
                result
            }
        };
    }
    pub fn main() {
        // let closure = |x| x + 1;
        //
        // closure(1);
        //
        // let mut range: Vec<i32> = (0..=100).collect();
        // range.retain(|x| x % 2 == 0);
        //
        // println!("{:?}", accept_closure(closure));

        // foo();
        // bar();
        //
        // print_result!(1u32 + 1);
        //
        // print_result!({
        //     let x = 1u32;
        //     x * x + 2 * x - 1
        // });
        //
        // println!("{:?}" , addition!(2,3));

        thread_main()
    }

    pub fn accept_closure(f: fn(i32) -> i32) -> i32 {
        f(2)
    }

    pub fn thread_main(){
        use std::thread;
        use std::time::Duration;

        let hello_handle = thread::spawn(||{
            loop {
                println!("Hello");
                thread::sleep(Duration::from_millis(1000));
            }
        });

        let world_handle = thread::spawn(||{
            loop {
                println!("World");
                thread::sleep(Duration::from_millis(1000));
            }
        });

        hello_handle.join().unwrap();
        world_handle.join().unwrap();

    }


}

#[cfg(test)]
mod chp7_tests {
    use crate::chp7_closure::chp7::accept_closure;
    use super::*;

    #[test]
    fn test_accept_closure(){
        let closure = |x| x * 1;
        assert_eq!(accept_closure(closure), 2);
    }

}

