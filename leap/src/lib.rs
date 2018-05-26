pub fn is_leap_year(year: i32) -> bool {

    // if (year % 4) == 0 {
    //     if (year % 100) == 0 {
    //         if (year % 400) == 0 { true } else { return false; };
    //     };
    //     return true;
    // };
    // false
    //

    // using match and if guard

    match year {
        year if (year % 400 == 0) => true,
        year if (year % 100 == 0) => false,
        year if (year % 4 == 0) => true,
        _ => false
    }

}
