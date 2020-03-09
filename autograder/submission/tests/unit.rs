use submission::solution::fib;

#[test]
fn test0() {
    assert_eq!(fib(0), 1, "YAY");
}

#[test]
fn test1() {
    assert_eq!(fib(1), 1);
}

#[test]
fn test2() {
    assert_eq!(fib(2), 2);
}

#[test]
fn test3() {
    assert_eq!(fib(3), 3);
}

#[test]
fn test4() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(fib(4), 5, "NOOOOOO");
}

#[test]
fn test5() {
    assert_eq!(fib(5), 8);
}

#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

//#[test]
//fn it_doesnt_work() -> Result<(), String> {
//if 2 + 2 == 3 {
//Ok(())
//} else {
//Err(String::from("two plus two does not equal three"))
//}
//}
