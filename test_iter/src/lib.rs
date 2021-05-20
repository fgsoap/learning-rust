#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

    let mut v2_iter = v1.into_iter();
    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
    assert_eq!(v2_iter.next(), Some(3));

    // let mut v3_iter = v1.iter_mut();
}
