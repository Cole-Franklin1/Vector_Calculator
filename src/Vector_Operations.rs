use std::fmt::{Formatter, Error};
use std::fmt;
use std::ops::{Range, Index, IndexMut};

pub fn scalar_multiply(vec1: &mut VectorND, scalar: f32) {
    for mut i in 0..vec1.len(){
        vec1[i] *= scalar;
    }
}

pub fn m_scalar_multiply(mat: &mut MatrixND, scalar: f32) {
    for i in 0..mat.len(){
        for mut j in 0..mat[i].len() {
            mat[i][j] *= scalar;
        }
    }
}

pub fn scalar_divide(vec1: &mut VectorND, scalar: f32) {
    for i in 0..vec1.len(){
        vec1[i] /= scalar;
    }
}

pub fn m_scalar_divide(mat: &mut MatrixND, scalar: f32) {
    for i in 0..mat.len(){
        for j in 0..mat[i].len() {
            mat[i][j] /= scalar;
        }
    }
}

pub fn dot(vec1: VectorND, vec2 : VectorND)->f32 {
    let mut prod = 0.0;
    for i in 0..vec1.components.len() {
        prod += vec1.components[i]*vec2.components[i];
    }
    prod
}

//pub fn cross(vec1: &mut Vec<i32>, vec2 : Vec<i32>) {
//
//}
//
//pub fn matrix_multiply(matrix1 : Vec<Vec<i32>>, matrix2 : Vec<Vec<i32>>){
//
//}

//matrix_formation!{

//}
//
// pub fn inverse_matrix(matrix1 : Vec<Vec<i32>>)

pub struct VectorND {
    pub components: Vec<f32>
}

impl std::ops::Add for VectorND{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert_eq!(self.components.len(), other.components.len());
        let mut out = Self {
            components: vec![0.; self.components.len()]
        };
        for i in 0..self.components.len(){
            out.components[i] = (self.components[i] + other.components[i]);
        }
        out
    }
}

impl fmt::Display for VectorND{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.components.len(){
            write!(f, "{:3} ",self.components[i]);
        }
        writeln!(f)
    }
}

impl VectorND{
    pub fn new(n: u8)->VectorND{
        VectorND{
            components: vec![0.; n as usize],
        }
    }
    pub fn len(&self)->usize{
        self.components.len()
    }
}

impl std::clone::Clone for VectorND{
    fn clone(&self) -> VectorND{
        let mut copy = VectorND::new(self.len() as u8);
        for i in 0..self.len(){
            copy[i] = self[i];
        }
        copy
    }
}

pub struct MatrixND {
    pub rows: Vec<VectorND>

}

impl MatrixND{
    pub fn new(r: u8, c: u8)->MatrixND{
        MatrixND{
            rows: vec![VectorND::new(c); r as usize],
        }
    }
    pub fn len(&self)->usize{
        self.rows.len()
    }
}

impl std::ops::Add for MatrixND{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert_eq!(self.rows.len(), other.rows.len());
        let mut out = MatrixND {
            rows: vec![VectorND::new(self.rows[0].components.len() as u8); self.rows.len()]
        };
        for i in 0..self.len(){
            for j in 0..self[i].len(){
                out[i][j] = self[i][j] + other[i][j];
            }
        }
        out
    }
}

impl fmt::Display for MatrixND{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len(){
            write!(f, "{} ",self[i]);
        }
        write!(f,"")
    }
}


pub fn det(mat: MatrixND)-> f32{
    let length = mat.len();
    assert_eq!(length, mat[0].len());
    for i in 1..length{
        assert_eq!(mat[0].len(),mat[i as usize].len());
    }
    let mut deter = 0.;
    if length == 2{
        deter += mat[0][0]*mat[1][1];
        deter -= mat[0][1]*mat[1][0];
    }
    else if length == 1{
        deter += mat[0][0]
    }
    else{
        let mut sign = 1.;
        for i in 0..length{
            let mut minor = MatrixND::new((length - 1) as u8, (length - 1) as u8);
            let mut row = 0;
            for j in 1..length{
                let mut col = 0;
                for k in 0..length{
                    if k != i{
                        minor[row][col] = mat[j][k];
                        col += 1;
                    }
                }
                row += 1;
            }
            deter += mat[0][i]*det(minor)*sign;
            sign *= -1.;
        }
    }
    deter as f32
}

impl Index<usize> for VectorND {
    type Output = f32;

    fn index(&self, ind:usize) -> &f32 {
        &self.components[ind]
    }
}

impl IndexMut<usize> for VectorND {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl Index<usize> for MatrixND {
    type Output = VectorND;

    fn index(&self, ind:usize) -> &VectorND {
        &self.rows[ind]
    }
}

impl IndexMut<usize> for MatrixND {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}


