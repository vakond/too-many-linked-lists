use lists::second;

#[test]
fn basics() {
    let mut list = second::List::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check peeks
    assert_eq!(list.peek_front(), Some(&3));
    assert_eq!(list.peek_front_mut(), Some(&mut 3));
    list.peek_front_mut().map(|val| *val = 33);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(33));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn into_iter() {
    let mut list = second::List::new();

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Use iterator
    let mut it = list.into_iter();
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), None);

    // Another instance
    list = second::List::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Use in for loop
    let values = [3, 2, 1];
    let mut i = 0;
    for val in list.into_iter() {
        assert_eq!(val, values[i]);
        i += 1;
    }
}

#[test]
fn iter() {
    let mut list = second::List::new();

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Use iterator
    let mut it = list.iter();
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), None);
}

#[test]
fn iter_mut() {
    let mut list = second::List::new();

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Use iterator
    let mut it = list.iter_mut();
    assert_eq!(it.next(), Some(&mut 3));
    assert_eq!(it.next(), Some(&mut 2));
    assert_eq!(it.next(), Some(&mut 1));
    assert_eq!(it.next(), None);
}
