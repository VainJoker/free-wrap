use free_wrap::Wrap;

// Define a wrapper for any type
Wrap!(i32, MyInt, derive = { Debug, Clone, PartialEq, Eq, PartialOrd, Ord });

fn main() {
    let a = MyInt::from(10);
    println!("{a:?}");
    let b: i32 = a.clone().into();
    println!("{b}");
    let c = a == MyInt::from(10);
    println!("{c}");
}
