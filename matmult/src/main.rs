//
// Simple Matrix Multiplication Program
//
// My first project in Rust to get a feel for the language.
//
// Eventually, I intend to paralleize the multiplication.
//

extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;

// Matrix type is defined to be a vector of vectors.  The
// outer vector defines the rows, which are represented
// as each respective inner vector, running top-down.
// NOTE: Most functions below depend on this representation
// of a matrix so changing this code will likely break
// all subsequent code.
// TODO add in some error checking somewhere to take care
// of cases where the rows/cols don't match data
#[derive(Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}

// Generates a matrix loaded with all zeros
fn genZeroMat(rows: usize, cols: usize) -> Matrix {
    let data = vec![vec![0.0f64; cols]; rows];
    Matrix{rows: rows, cols: cols, data: data}
}

// TODO change the funciton to it gens ints and
// specify the range for these rand values rather than
// the default of (0,1)
fn genRandMat(rows: usize, cols: usize) -> Matrix {
    let mut result = genZeroMat(rows, cols);

    for y in 0..rows {
        for x in 0..cols {
            result.data[y][x] =
                rand::random::<f64>();
        }
    }

    return result;
}

// Multiplies a matrix by a constant
fn const_mult(A: Matrix, k: f64) -> Matrix {
    let mut result = genZeroMat(A.rows, A.cols);

    for y in 0..A.rows {
        for x in 0..A.cols {
            result.data[y][x] = k * A.data[y][x];
        }
    }

    return result;
}

// Adds two matrices.
// Panics if the two matrices aren't the same dims.
fn add(A: Matrix, B: Matrix) -> Matrix {
    if A.rows != B.rows || A.cols != B.cols {
        panic!("Matrix dimensions don't match.");
    }

    let mut result = genZeroMat(A.rows, A.cols);

    for y in 0..A.rows {
        for x in 0..A.cols {
            result.data[y][x] = A.data[y][x] + B.data[y][x];
        }
    }

    return result;
}

// Reduces matrix into four separate blocks such that
// we obtain:
// 0,0 | 0,1
// ----------
// 1,0 | 1,1
//
// TODO handle the case where n is odd
fn quarter(A:Matrix) -> Vec<Vec<Matrix>> {
    let newsize = A.rows / 2;
    let mut result = vec![vec![genZeroMat(newsize
        , newsize); 2]; 2];

    for y in 0..newsize {
        for x in 0..newsize {
            result[0][0].data[y][x] = A.data[y][x];
            result[1][0].data[y][x] = A.data[newsize + y][x];
            result[0][1].data[y][x] = A.data[y][newsize + x];
            result[1][1].data[y][x]
                = A.data[newsize + y][newsize + x];
        }
    }

    return result;
}

// Implementation of Strassen's algorithm.
fn strassen(A: Matrix, B: Matrix) -> Matrix {
    let Aquarters = quarter(A);
    let Bquarters = quarter(B);

    unimplemented!();
}

// Multiplies matrix A by B and returns Some(A*B).
// Standard method of multiplication.
// If the size of the matrices aren't compatible,
// then returns None.
fn mult(A: Matrix, B: Matrix)
    -> Option<Matrix> {
    if A.cols != B.rows {
        println!("{:?}\n", A.data);
        println!("{:?}\n", B.data);
        None
    } else {
        // TODO CHANGE THE DEFAULT DATA!
        let mut C = genZeroMat(A.rows, B.cols);
        for Arow in 0..A.rows {
            for Acol in 0..A.cols {
                for Bcol in 0..B.cols {
                    C.data[Arow][Bcol] += A.data[Arow][Acol]
                        * B.data[Acol][Bcol]
                }
            }
        }
        return Some(C);
    }
}

// Multiplies matrix A by B and returns Some(A*B).
// Multiplies the matrices in parallel across all
// processors on the current machine.
// If the size of the matrices aren't compatible,
// then returns None.
fn pmult(A: Matrix, B: Matrix) -> Option<Matrix> {
    thread::spawn(move || {
        test1();
    });

    test1;

    None
}

fn test1() {
    let TestA = Matrix{rows: 2, cols: 2
        , data: vec![vec![1.0f64, 0.0f64],vec![0.0f64, 1.0f64]]};
    let TestB = Matrix{rows: 2, cols: 2
        , data: vec![vec![1.0f64, 2.0f64],vec![3.0f64, 4.0f64]]};
    // println!("{:?}\n", TestA.cols);
    // println!("{:?}\n", TestB.rows );
    // println!("{:?}\n", TestA.cols != TestB.rows );
    match mult(TestA, TestB) {
        Some(result) => println!("{:?}"
            ,result.data),
        None => println!("Function failed. Sad!"),
    }

}

fn test2() {
    let TestA = Matrix{rows: 4, cols: 4
        , data: vec![vec![1.0f64, 2.0f64, 3.0f64, 4.0f64]
        ,vec![5.0f64, 6.0f64, 7.0f64, 8.0f64]
        ,vec![9.0f64, 10.0f64, 11.0f64, 12.0f64]
        ,vec![13.0f64, 14.0f64, 15.0f64, 16.0f64]]};
    let quarters = quarter(TestA);
    println!("{:?},",quarters[0][0].data);
    println!("{:?},",quarters[1][0].data);
    println!("{:?},",quarters[0][1].data);
    println!("{:?},",quarters[1][1].data);
}

fn test_strassen() {
    // do stuff
}

fn test_parallel() {
    let x = 1;

    thread::spawn(move || {
        println!("x is {}", x);
    });
    thread::sleep(Duration::from_millis(50));
}

fn main() {
    let TestA = Matrix{rows: 2, cols: 2
        , data: vec![vec![1.0f64, 0.0f64],vec![0.0f64, 1.0f64]]};
    let TestB = Matrix{rows: 2, cols: 2
        , data: vec![vec![1.0f64, 2.0f64],vec![3.0f64, 4.0f64]]};

    // let handle1 = thread::spawn(|| {
    //     quarter(TestA)
    // });
    // let handle2 = thread::spawn(|| {
    //     quarter(TestB)
    // });
    //
    // let Aquarters = handle1.join().unwrap();
    // let Bquarters = handle2.join().unwrap();
    let Aquarters = quarter(TestA);
    let Bquarters = quarter(TestB);

    /// DEBUG CODE
    println!("{:?},", Bquarters[0][0].data);
    println!("{:?},", Bquarters[1][0].data);
    println!("{:?},", Bquarters[0][1].data);
    println!("{:?}", Bquarters[1][1].data);

    // let mut result = genZeroMat(2,2);
    let mut resultQuarters = quarter(genZeroMat(2,2));

    for i in 0..1 {
        for j in 0..1 {
            for k in 0..1 {
                let temp = resultQuarters[i][j].clone();
                let A = Aquarters[i][j].clone();
                let B = Bquarters[j][k].copy();

                resultQuarters[i][k] = add(temp
                    , B);//mult(A, B).unwrap());
            }
        }
    }

    // final step is to print each of the quaters
    // and/or join them all together again.
    println!("{:?},",resultQuarters[0][0].data);
    println!("{:?},",resultQuarters[1][0].data);
    println!("{:?},",resultQuarters[0][1].data);
    println!("{:?},",resultQuarters[1][1].data);
}
