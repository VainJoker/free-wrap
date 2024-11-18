use free_wrap::Wrap;

#[test]
const fn test_simple() {
    Wrap!(i32, MyInt);
    impl From<MyInt> for i64 {
        fn from(v: MyInt) -> Self {
            Self::from(v.0)
        }
    }
}

#[test]
const fn test_with_derive() {
    Wrap!(i32, MyInt, derive = { Debug, Clone, PartialEq, Eq, PartialOrd, Ord });
}
