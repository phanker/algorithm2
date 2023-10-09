fn main() {
    /**
    1.brake down the regular
    2.observe the change between x-axis and y-axis
    3.obtain the regular
    x <= 2 | y <= 2 | n = 3


    x = y
    y = n - x - 1


    0  0    1  0   2  0
     |       |       |
    0  2    0  1   0  0
     */


    //1,2 => 2,1
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - 1 - i {
                let num_1 = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = matrix[i][j];

                let num_2 = matrix[n - i - 1][n - 1 - j];
                matrix[n - i - 1][n - 1 - j] = num_1;

                let num_3 = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = num_2;

                matrix[i][j] = num_3;
            }
        }
    }
}
