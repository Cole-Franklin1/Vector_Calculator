use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::string::ParseError;

pub fn scalar_multiply(vec: & VectorND, scalar: f32)->VectorND {
    let mut out = VectorND::new(vec.len() as u8);
    for mut i in 0..vec.len(){
        out[i] = vec[i] * scalar;
    }
    out
}

pub fn m_scalar_multiply(mat: & MatrixND, scalar: f32)->MatrixND {
    let mut out = MatrixND::new(mat.len() as u8, mat[0].len() as u8);
    for i in 0..mat.len(){
        out[i] = scalar_multiply(& mat[i],scalar);
    }
    out
}

pub fn scalar_divide(vec: &VectorND, scalar: f32)->VectorND {
    let mut out = VectorND::new(vec.len() as u8);
    for mut i in 0..vec.len(){
        out[i] = vec[i] / scalar;
    }
    out
}

pub fn m_scalar_divide(mat: &MatrixND, scalar: f32)->MatrixND {
    let mut out = MatrixND::new(mat.len() as u8, mat[0].len() as u8);
    for i in 0..mat.len(){
        out[i] = scalar_divide(&mat[i],scalar);
    }
    out
}

pub fn dot(vec1: &VectorND, vec2 : &VectorND)->Result<f32, &'static str> {
    if vec1.len() != vec2.len(){
        let t : Result<_,_> = Err("Vectors must be the same length to have a dot product");
        return t
    }
    let mut prod = 0.0;
    for i in 0..vec1.len() {
        prod += vec1[i]*vec2[i];
    }
    Ok(prod)
}

pub fn cross(vec1: VectorND, vec2 : VectorND)-> Result<VectorND, &'static str> {
    if vec1.len() != vec2.len() || vec1.len() != 3{
        let t : Result<_,_> = Err("Vectors must both be 3 dimensional to take a cross product");
        return t
    }
    let mut out = VectorND::new(3);
    for i in 0..3{
        let mut minor = MatrixND::new(2, 2);
        let mut row = 0;
        for j in 1..3 {
            let mut col = 0;
            for k in 0..3 {
                if k != i {
                    minor[row][col] = match j{
                        1 => vec1[k],
                        2 => vec2[k],
                        _ => 0.
                    };
                    col += 1;
                }
            }
            row += 1;
        }
        if let Ok(val) = det(&minor) {
            out[i] = val;
        }
    }
    Ok(out)
}

pub fn matrix_multiply(matrix1 : MatrixND, matrix2 : MatrixND)->Result<MatrixND, &'static str> {
    if matrix1[0].len() != matrix2.len(){
        let t : Result<_,_> = Err("This matrix multiplication is undefined.");
        return t
    }
    let mut out = MatrixND::new(matrix1.len() as u8, matrix2[0].len() as u8);
    let mut matrix2_t = MatrixND::new(matrix2[0].len() as u8, matrix2.len() as u8);
    for i in 0..matrix2[0].len(){
        for j in 0.. matrix2.len(){
            matrix2_t[i][j] = matrix2[j][i];
        }
    }
    for i in 0..matrix1.len(){
        for j in 0..matrix1[0].len(){
            if let Ok(val) = dot(&matrix1[i],&matrix2_t[j]) {
                out[i][j] =val;
            }
        }
    }
    Ok(out)
}

pub fn det(mat: &MatrixND)-> Result<f32, &'static str>{
    let length = mat.len();
    if length != mat[0].len(){
        let t : Result<_,_> = Err("This determinant is undefined, only determinants of square matrices are defined.");
        return t
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
            if let Ok(val) = det(&minor) {
                deter += mat[0][i]*val*sign;
            }
            sign *= -1.;
        }
    }
    Ok(deter as f32)
}

 pub fn inverse_matrix(mat : &MatrixND)->Result<MatrixND,&'static str>{
     let length = mat.len();
     let det1 = det(&mat);
     if let Err(t) = det1{
        return Err("The determinant is undefined, therefore the inverse in undefined, only determinants of square matrices are defined.")
     }
     let mut d = 0.;
     if let Ok(t) = det1 {
         if t == 0.{
            return Err("The determinant is 0, therefore the inverse in undefined.")
         }
         d = t;
     }
     let mut out = MatrixND::new(length as u8, length as u8);
     let mut sign = 1.;
     for i in 0..length{
         if i % 2 == 0{ sign = 1.;}
         else{ sign = -1.;}
         for j in 0..length{
            let mut minor = MatrixND::new((length - 1) as u8, (length - 1) as u8);
            let mut row = 0;
            for k in 0..length{
                let mut col = 0;
                if k != i{
                    for l in 0..length{
                        if l != j{
                            minor[row][col] = mat[k][l];
                            col += 1;
                        }
                    }
                    row += 1;
                }
            }
            if let Ok(deter) = det(&minor){
                out[i][j] = deter*sign;
            }
            sign *= -1.;
        }
        }
     let mut pre_t = m_scalar_divide(&out,d);
     let mut transposed = MatrixND::new(length as u8, length as u8);
     for i in 0..length{
         for j in 0..length{
             transposed[j][i] = pre_t[i][j];
         }
     }
     Ok(transposed)
}

pub struct VectorND {
    pub components: Vec<f32>
}

impl std::ops::Add for VectorND{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert_eq!(self.len(), other.len());
        let mut out = Self {
            components: vec![0.; self.len()]
        };
        for i in 0..self.len(){
            out[i] = self[i] + other[i];
        }
        out
    }
}

impl fmt::Display for VectorND{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len(){
            if i != 0{
                write!(f,",");
            }
            if i == 0{
                write!(f,"[{:2}",self[i]);
                continue;
            }
            write!(f, "{:2}",self[i]);
        }
        write!(f,"]")
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
        assert_eq!(self.len(), other.len());
        assert_eq!(self[0].len(), other[0].len());
        let mut out = MatrixND {
            rows: vec![VectorND::new(self[0].len() as u8); self.len()]
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
            writeln!(f, "{}",self[i]);
        }
        write!(f,"")
    }
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

impl FromStr for VectorND{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut index = 0;
        let mut cur_num:String = "".to_string();
        let mut v = vec![];
        let mut err = false;
        let mut open_vec = false;
        let mut polar_form = false;
        for i in s.chars(){
            if index > 0 || i != ' '{
                if i == '[' && open_vec == false{
                    open_vec = true;
                }
                else if i == '['{
                    err = true;
                    break;
                } else if i == '(' && open_vec == false{
                    open_vec = true;
                    polar_form = true;
                }
                else if i == '('{
                    err = true;
                    break;
                }
                if i == ' ' || i == ',' || i == ']'|| i == ')'{
                    if open_vec {
                        if cur_num != "".to_string() && cur_num != " ".to_string() {
                            if let Ok(val) = cur_num.parse::<f32>() {
                                v.push(val);
                                cur_num = "".to_string();
                            }
                        }else{
                            continue;
                        }
                    }
                }else if i.is_numeric() || i == '.' {
                    let temp = format!("{}{}", cur_num, i);
                    cur_num = temp;
                }
                index += 1;
                } if i == ']' || i ==')'{
                    open_vec = false

            }
        }
        let mut out = VectorND::new(0);
        if polar_form{
            if v.len() == 2{
                out.components.push(v[0]*v[1].cos());
                out.components.push(v[0]*v[1].sin());
                return Ok(out)
            }else if v.len() == 3{
                let r = v[0] * v[2].sin();
                out.components.push(r*v[1].cos());
                out.components.push(r*v[1].sin());
                out.components.push(v[0]*v[2].cos());
                return Ok(out)
            }else{
                err = true;
            }
        }
        if err{return Ok(out)};
        out.components = v;
        Ok(out)
    }
}

impl FromStr for MatrixND{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = MatrixND::new(1, 1);
        let mut index = 0;
        let mut cur_num : String = "".to_string();
        let mut m = vec![];
        let mut err = false;
        let mut open_mat = false;
        let mut open_vec = false;
        let mut v = vec![];
        let p = &mut v;
        for i in s.chars() {
            if index > 0 || i != ' ' {
                if i == '<' && open_mat == false {
                    open_mat = true;
                    index += 1;
                } else if i == '<' {
                    err = true;
                    break;
                }
                if i == '[' && open_vec == false{
                    open_vec = true;
                    *p = vec![];
                }
                else if i == '['{
                    err = true;
                    break;
                }
                if i == ' ' || i == ',' || i == ']' {
                    if open_vec {
                        if cur_num != "".to_string() && cur_num != " ".to_string() {
                            if let Ok(val) = cur_num.parse::<f32>() {
                                p.push(val);
                                cur_num = "".to_string();
                            }
                        }else{
                            continue;
                        }
                    }
                } else if i.is_numeric() || i == '.' {
                    let temp = format!("{}{}", cur_num, i);
                    cur_num = temp;
                }
                if i == ']' {
                    let mut entry = VectorND::new(0);
                    for i in p.iter(){
                        entry.components.push(*i);
                    }
                    open_vec = false;
                    m.push(entry);
                }
                if i == '>' {
                    open_mat = false
                }
            }
        }
        if err{return Ok(out)};
        out.rows = m;
        Ok(out)

    }
}