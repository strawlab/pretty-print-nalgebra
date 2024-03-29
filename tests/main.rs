extern crate nalgebra as na;
#[macro_use]
extern crate pretty_print_nalgebra;

use na::Matrix2;

#[test]
fn it_works() {
    let arr = Matrix2::<f64>::new(1.0, 2.0, 3.0, 4.0);
    let arr_str: String = pretty_print!(arr);
    assert_eq!(&arr_str, "\n           1.000        2.000\n           3.000        4.000");
}
