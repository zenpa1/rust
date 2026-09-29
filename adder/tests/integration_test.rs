use adder::add;

#[test]
fn it_adds_2() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
