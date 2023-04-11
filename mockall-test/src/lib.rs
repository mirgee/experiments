trait MyTrait {
    fn do_something(&self) -> u32;
}

pub struct MyStruct {}

impl MyTrait for MyStruct {
    fn do_something(&self) -> u32 {
        123
    }
}

#[test]
fn test_something() {
    use mockall::mock;

    mock! {
        pub MyStruct {}

        impl MyTrait for MyStruct {
            fn do_something(&self) -> u32 {
                42
            }
        }
    }

    let mut mock = MockMyStruct::new();
    mock.expect_do_something().returning(|| 42);
    assert_eq!(mock.do_something(), 42);
}
