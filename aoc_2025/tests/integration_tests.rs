mod test_util;

#[test]
fn test_day_1() {
    test_util::test_day_part(1, 1, "1150");
    test_util::test_day_part(1, 2, "6738");
}

#[test]
fn test_day_2() {
    test_util::test_day_part(2, 1, "21898734247");
    test_util::test_day_part(2, 2, "28915664389");
}

#[test]
fn test_day_3() {
    test_util::test_day_part(3, 1, "17493");
    test_util::test_day_part(3, 2, "173685428989126");
}

#[test]
fn test_day_4() {
    test_util::test_day_part(4, 1, "1370");
    test_util::test_day_part(4, 2, "8437");
}
