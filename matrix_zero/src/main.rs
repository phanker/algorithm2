fn main() {
    println!("Hello, world!");

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        // firstly determine the first row and first column is 0
        let mut first_row_zero = false;
        let mut first_column_zero = false;

        // iterate rows
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_row_zero = true;
                break;
            }
        }
        for i in 0..n {
            if matrix[0][i] == 0 {
                first_column_zero = true;
                break;
            }
        }


        // make the marks on the first row and column
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }


        //according to the marks to determine if the values of element are needed to change as 0
        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }

        for i in 1..n {
            if matrix[0][i] == 0 {
                for j in 1..m {
                    matrix[j][i] = 0;
                }
            }
        }

        if first_column_zero == true {
            for i in 0..n {
                matrix[0][i] = 0;
            }
        }

        if first_row_zero == true {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}
