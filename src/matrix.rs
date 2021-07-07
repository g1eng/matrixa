// Matrix trait
//
//
pub trait Matrix {
    fn print(&self);
}

pub trait TensorProcessor {
    fn add(&mut self, val: i32) -> &mut Self;
    fn substract(&mut self, val: i32) -> &mut Self;
    fn multiple(&mut self, val: i32) -> &mut Self;
    fn divide(&mut self, val: i32) -> &mut Self;
    fn residue(&mut self, val: i32) -> &mut Self;
    fn check_zero_len(&self);
    fn replace_row(&mut self, src: usize, dst: usize) -> &mut Self;
    fn replace_col(&mut self, src: usize, dst: usize) -> &mut Self;
    fn translate(&mut self) -> &mut Self;

}

// I32Matrix
//
//
pub struct I32Matrix {
    pub data: Vec<Vec<i32>>,
    current: usize,
    max: usize,
}

impl I32Matrix {
    pub fn new() -> Self {
        I32Matrix {
            data: Vec::new(),
            current: 0,
            max: 0,
        }
    }
    pub fn push(&mut self, data: Vec<i32>) -> &mut Self {
        self.max += 1;
        let rowlen = self.data.len();

        if rowlen != 0 {
            if self.data[0].len() != data.len(){
                panic!("Invalid vector length: {}, expected: {}",data.len(), rowlen);
            }
        }
        self.data.push(data);
        self
    }
}

impl Matrix for I32Matrix {
    fn print(&self){
        println!("{:?}",self.data)
    }
}

impl Iterator for I32Matrix {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Vec<i32>> {
        self.current += 1;
        if self.current - 1 < self.max {
            let data = &self.data[self.current-1];
            Some(data.to_vec())
        } else {
            None
        }
    }
}

impl TensorProcessor for I32Matrix {

    //行列サイズ確認
    // 長さ0で強制終了
    fn check_zero_len(&self) {
        if self.data.len() == 0 {
            panic!("zero matrix length detected");
        }
    }

    //一括加算
    //
    fn add(&mut self, val: i32) -> &mut Self {
        self.check_zero_len();
        for i in 0 .. self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] += val;
            }
        }
        self
    }

    //一括減算
    //
    fn substract(&mut self, val: i32) -> &mut Self {
        self.check_zero_len();
        for i in 0 .. self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] -= val;
            }
        }
        self
    }

    //一括乗算
    //
    fn multiple(&mut self, val: i32) -> &mut Self {
        self.check_zero_len();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] *= val;
            }
        }
        self
    }

    //一括除算
    // (i32デフォルトで端数切捨て)
    fn divide(&mut self, val: i32) -> &mut Self {
        self.check_zero_len();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] /= val;
            }
        }
        self
    }

    //一括剰余
    //
    fn residue(&mut self, val: i32) -> &mut Self {
        self.check_zero_len();
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                self.data[i][j] %= val;
            }
        }
        self
    }

    //行置換操作
    //
    fn replace_row(&mut self, src: usize, dst: usize) -> &mut Self {
        self.check_zero_len();

        if src >= self.data.len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data.len() {
            panic!("dst address out of range {}", dst);
        }


        let mut src_data: Vec<i32> = Vec::new();
        let mut dst_data: Vec<i32> = Vec::new();

        for i in 0..self.data.len() {
            src_data.push(self.data[src][i]);
            dst_data.push(self.data[dst][i]);
        }
        self.data[src] = dst_data;
        self.data[dst] = src_data;
        self
    }

    //列置換操作
    //
    fn replace_col(&mut self, src: usize, dst: usize) -> &mut Self {
        self.check_zero_len();

        if src >= self.data[0].len() {
            panic!("src address out of range {}", src);
        } else if dst >= self.data[0].len() {
            panic!("dst address out of range {}", dst);
        }

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                let src_data = self.data[i][src];
                let dst_data = self.data[i][dst];
                self.data[i][src] = dst_data;
                self.data[i][dst] = src_data;
            }
        }
        self

    }

    // 転置
    //
    fn translate (&mut self) -> &mut Self {
        self.check_zero_len();
        if self.data.len() != self.data[0].len() {
            panic!("not a square matrix");
        }
        let limit = self.data.len() / 2;

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                if (i<=limit && j >= limit){
                    let src = self.data[i][j];
                    let dst = self.data[j][i];
                    self.data[i][j] = dst;
                    self.data[j][i] = src;
                }
            }
        }
        self
   }
}
















// F32 Matrix
//
//
pub struct F32Matrix {
    pub data: Vec<Vec<f32>>,
    current: usize,
    max: usize,
}

impl F32Matrix {
    pub fn new() -> Self {
        F32Matrix {
            data: Vec::new(),
            current: 0,
            max: 0,
        }
    }
    pub fn push(&mut self, data: Vec<f32>) -> &mut Self {
        self.max += 1;
        let rowlen = self.data.len();

        if rowlen != 0 {
            if rowlen != data.len(){
                panic!("Invalid vector length: {}, expected: {}",data.len(), rowlen);
            }
        }
        self.data.push(data);
        self
    }
}

impl Matrix for F32Matrix {
    fn print(&self){
        println!("{:?}",self.data)
    }
}

impl Iterator for F32Matrix {
    type Item = Vec<f32>;
    fn next(&mut self) -> Option<Vec<f32>> {
        self.current += 1;
        if self.current - 1 < self.max {
            let data = &self.data[self.current-1];
            Some(data.to_vec())
        } else {
            None
        }
    }
}

