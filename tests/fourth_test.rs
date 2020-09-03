use lists::fourth;

#[test]
fn basics() {
    let mut list = fourth::List::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_back(100);

    // Check peeks
    assert_eq!(*list.peek_front().unwrap(), 3);
    assert_eq!(*list.peek_front_mut().unwrap(), 3);
    list.peek_front_mut().map(|mut val| *val = 42);
    assert_eq!(*list.peek_back().unwrap(), 100);
    assert_eq!(*list.peek_back_mut().unwrap(), 100);
    list.peek_back_mut().map(|mut val| *val = 142);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(42));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(142));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);
    list.push_back(100);
    list.push_back(200);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(100));
    assert_eq!(list.pop_front(), Some(200));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn iterator() {
    let mut list = fourth::List::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    let mut s = String::new();
    for i in list.into_iter() {
        s += format!(" {}", i).as_str();
    }

    assert_eq!(s, " 1 2 3 4");
}
