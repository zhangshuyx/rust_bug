struct Foo {
    x: i32,
}

fn main() {
    let mut foo = Foo { x: 42 };
    //let x = &mut foo.x;
    //*x = 13;
    foo.x = 22;
    let y = foo;
    println!("{}", y.x); // -> 42; expected result: 13
}
