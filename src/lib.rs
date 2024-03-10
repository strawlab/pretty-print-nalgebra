//! a small macro-only crate providing ability to pretty-print a 2D nalgebra array

#[macro_export]
macro_rules! pretty_print {
    ($arr: expr) => {
        pretty_print!($arr, 4, 12, 3)

    };
    ($arr: expr, $indent:expr,) => {
        pretty_print!($arr, $indent, 12, 3)

    };
    ($arr: expr, $indent:expr, $width:expr) => {
        pretty_print!($arr, $indent, $width, 3)

    };
    ($arr:expr, $indent:expr, $width:expr, $precision: expr) => {{
        let prefix = String::from_utf8(vec![b' '; $indent]).unwrap();
        let mut result_els = vec!["".to_string()];
        let width = $width;
        let precision = $precision;
        for i in 0..$arr.nrows() {
            let mut row_els = vec![];
            for j in 0..$arr.ncols() {
                row_els.push(format!("{:width$.precision$}", $arr[(i,j)]));
            }
            let row_str = row_els.into_iter().collect::<Vec<_>>().join(" ");
            let row_str = format!("{}{}", prefix, row_str);
            result_els.push( row_str );
        }
        result_els.into_iter().collect::<Vec<_>>().join("\n")
    }}
}
