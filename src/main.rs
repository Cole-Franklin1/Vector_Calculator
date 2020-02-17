use read_input::prelude::*;
use crate::Vector_Operations::{scalar_multiply, m_scalar_multiply, dot, VectorND, MatrixND, det, matrix_multiply, cross, scalar_divide, m_scalar_divide, inverse_matrix};
mod Vector_Operations;

//This is a school project.
//If anyone wants to use part of this project, that is fine, but there are libraries that already accomplish this.

fn main() {
    println!("Hello, welcome to my vector calculator! \nIf you want to write a vector in spherical/polar form use the format (magnitude, direction1, direction2) with radians.\n\
    If you want to use component form use the format [component1, component2, component3,..].\n\
    If you want to input a matrix use the format <[row1],[row2], ...> with each row having the form of a component vector.\n\
    det(matrix) = determinant\n\
    vector*vector = dot product =/= vector x vector = cross product\n\
    NOTE: cross product is only implemented for 3d vectors\n");
    let mut op : u8 = input().msg("Which operation would you like to run?\n\
    1. Addition  2. Subtraction 3. Scalar multiplication    4. Scalar division\n\
    5. Dot product 6. Cross product  7. Matrix multiplication   8. Determinant\n\
    9. Inverse Matrix").get();
    while op == 0 || op > 9{
        op = input().msg("I don't understand.\n").get();
    }
    while op != 0{
        let mut kind = 0;
        match op{
            1 | 2 | 3 | 4 => while kind != 1 && kind != 2{kind = input().msg("Would you like to input vectors(1) or matrices(2)?").get()},
            _ => kind = 0
        }
        let mut f_result : Result<f32,&str> = Err("Unused");
        let mut v_result : Result<VectorND,&str> = Err("Unused");
        let mut m_result : Result<MatrixND,&str> = Err("Unused");
        if op == 1{
            if kind == 1{
                let vec1 : VectorND = input().msg("Input vector 1: ").get();
                let vec2: VectorND = input().msg("\nInput vector 2: ").get();
                v_result = Ok(vec1 + vec2);
            }else{
                let mat1 : MatrixND = input().msg("Input Matrix 1: ").get();
                println!("{}",mat1);
                let mat2 : MatrixND = input().msg("\nInput Matrix 2: ").get();
                println!("{}",mat2);
                m_result = Ok(mat1 + mat2);
            }
        } else if op == 2{
            if kind == 1{
                let vec1 : VectorND = input().msg("Input vector 1: ").get();
                let vec2: VectorND = input().msg("\nInput vector 2: ").get();
                v_result = Ok(vec1 + scalar_multiply(&vec2,-1.));
            }else{
                let mat1 : MatrixND = input().msg("Input Matrix 1: ").get();
                let mat2 : MatrixND = input().msg("\nInput Matrix 2: ").get();
                m_result = Ok(mat1 + m_scalar_multiply(&mat2,-1.));
            }
        }else if op == 3{
            if kind == 1{
                let vec : VectorND = input().msg("Input vector 1: ").get();
                let scalar : f32 = input().msg("\nInput scalar:").get();
                v_result = Ok(scalar_multiply(&vec,scalar));
            }else{
                let mat : MatrixND = input().msg("Input Matrix 1: ").get();
                let scalar = input().msg("\nInput scalar: ").get();
                m_result = Ok(m_scalar_multiply(&mat,scalar));
            }
        }else if op == 4{
            if kind == 1{
                let vec : VectorND = input().msg("Input vector 1: ").get();
                let scalar : f32 = input().msg("\nInput scalar:").get();
                v_result = Ok(scalar_divide(&vec,scalar));
            }else{
                let mat : MatrixND = input().msg("Input Matrix 1: ").get();
                let scalar = input().msg("\nInput scalar: ").get();
                m_result = Ok(m_scalar_divide(&mat,scalar));
            }
        }else if op == 5{
            let vec1 : VectorND = input().msg("Input vector 1: ").get();
            let vec2: VectorND = input().msg("\nInput vector 2: ").get();
            f_result = dot(&vec1,&vec2);
        }else if op == 6{
            let vec1 : VectorND = input().msg("Both vectors must be of length 3.\nInput vector 1: ").get();
            let vec2: VectorND = input().msg("\nInput vector 2: ").get();
            v_result = cross(vec1,vec2);
        }else if op == 7{
            let mat1 : MatrixND = input().msg("The length of the 1st matrix's rows must be equal to the number of rows of the 2nd matrix.\nInput Matrix 1: ").get();
            let mat2: MatrixND = input().msg("\nInput Matrix 2: ").get();
            m_result = matrix_multiply(mat1,mat2);
        }else if op == 8{
            let mat : MatrixND = input().msg("The matrix must be square (nxn).\nInput Matrix: ").get();
            f_result = det(&mat);
        }else if op == 9{
            let mat: MatrixND = input().msg("The matrix must be square (nxn) with non-zero determinant.\nInput Matrix: ").get();
            m_result = inverse_matrix(&mat);
        }else{
            f_result = Err("oops");
        }
        if let Err(res) = f_result{if res != "Unused" {
            println!("{}", res)
        } }else if let Ok(answer) = f_result{
            println!("= \n{}", answer);
        }
        if let Err(res) = v_result{if res != "Unused" {
             println!("{}", res);
        }}else if let Ok(answer) = v_result{
            println!("= \n{}", answer);
        }
        if let Err(res) = m_result{if res != "Unused" {
            println!("{}", res);
        }}else if let Ok(answer) = m_result{
            println!("= \n{}", answer);
        }
        op = input().msg("Which operation would you like to run?\n\
    1. Addition  2. Subtraction 3. Scalar multiplication    4. Scalar division\n\
    5. Dot product 6. Cross product    7. Matrix multiplication   8. Determinant\n\
    9. Inverse matrix \n\
    OR you could type 0 to end the program").get();
        while op > 9{
            op = input().msg("I don't understand.").get();
        }

    }
}
