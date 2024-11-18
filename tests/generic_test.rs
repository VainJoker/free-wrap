use free_wrap::Wrap;

struct Container<T> {
    _content: T,
}

struct Container2<T: Clone + std::fmt::Debug, U> {
    _first:  T,
    _second: U,
}

struct Container3<'a: 'b + 'c, 'b: 'c, 'c> {
    _first:  &'a i32,
    _second: &'b i32,
    _third:  &'c i32,
}

struct Container4<'a: 'b + 'c, 'b: 'c, 'c, T, U> {
    _first:  &'a T,
    _second: &'b U,
    _third:  &'c i32,
}

#[test]
const fn test_wrap() {
    Wrap!(Container<T>, MyContainer1);
    Wrap!(Container2<T: Clone + std::fmt::Debug, U: Default>, MyContainer2);
    Wrap!(Container2<T: Clone + std::fmt::Debug, U>, MyContainer21);
    Wrap!(Container3<'a: 'b + 'c, 'b: 'c, 'c>, MyContainer3);
    Wrap!(Container4<'a, 'b, 'c, T, U>, MyContainer4);
    Wrap!(Container4<'a, 'b, 'c, T: Clone, U: Default>, MyContainer41);
    Wrap!(Container4<'a: 'b + 'c, 'b: 'c, 'c, T, U>, MyContainer42);
    Wrap!(Container4<'a: 'b + 'c, 'b: 'c, 'c, T: Clone + std::fmt::Debug, U: Default>, MyContainer43);
}
