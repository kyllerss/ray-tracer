// Happy coincidence the underlying implementations are identical:
// taken from https://athemathmo.github.io/rulinalg/doc/src/rulinalg/macros/matrix.rs.html#45-66
#[macro_export]
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::domain::matrix::Matrix;
            Matrix::new(0, 0, vec![])
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::domain::matrix::Matrix;
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let data_as_flat_array: Vec<_> = data_as_nested_array.iter()
                .flat_map(|row| row.iter())
                .cloned()
                .collect();
            Matrix::new(cols, rows, data_as_flat_array)
        }
    }
}
