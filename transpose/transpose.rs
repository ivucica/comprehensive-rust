fn transpose<const M : usize, const N : usize>(matrix: [[i32; M]; N]) -> [[i32; N]; M] {
    let mut new_rows : [[i32; N]; M] = [[0; N]; M];
    for new_col_id in 0..matrix.len() {
        for new_row_id in 0..matrix[new_col_id].len() {
            new_rows[new_row_id][new_col_id] = matrix[new_col_id][new_row_id];
        }
    }
    new_rows
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

#[test]
fn test_transpose_nonsquare() {
    let matrix = [
        [101, 102, 103, 104], //
        [201, 202, 203, 204],
        [301, 302, 303, 304],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
            [104, 204, 304],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}