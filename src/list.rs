use crate::core::Matrix;
use crate::mat;

impl Matrix<String> {

    fn contains(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].contains(pat))
            }
        }
        res
    }

    fn starts_with(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].starts_with(pat))
            }
        }
        res
    }

    fn ends_with(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].ends_with(pat))
            }
        }
        res
    }

    fn is_empty(&self) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].is_empty())
            }
        }
        res
    }

    fn is_ascii(&self) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].is_ascii())
            }
        }
        res
    }

    fn pop_char(&mut self) -> &Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].pop();
            }
        }
        self
    }

    fn push_char(&mut self, ch: char) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].push(ch);
            }
        }
    }

    fn push_str(&mut self, s: &str) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].push_str(s);
            }
        }
    }

    fn shrink_to(&mut self, min_capacity: usize) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].shrink_to(min_capacity);
            }
        }
    }

    fn replace(&mut self, from: &str, to: &str) -> &Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].replace(from, to);
            }
        }
        self
    }

    fn to_strlen(&self) -> Matrix<usize> {
        let mut res = mat![usize];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].len())
            }
        }
        res
    }

    fn trim_start(&self) -> Matrix<String> {
        let mut res = mat![String];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(String::from(self.data[i][j].trim_start()))
            }
        }
        res
    }

    fn trim_end(&self) -> Matrix<String> {
        let mut res = mat![String];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(String::from(self.data[i][j].trim_end()))
            }
        }
        res
    }

    fn trim(&self) -> Matrix<String> {
        let mut res = mat![String];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(String::from(self.data[i][j].trim()));
            }
        }
        res
    }

    fn as_bytes(&self) -> Matrix<&[u8]>{
        let mut res = mat![&[u8]];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].as_bytes())
            }
        }
        res
    }
}

#[cfg(test)]
mod tests_matrix_conversion {
    use crate::core::Matrix;
    use crate::mat;

    #[test]
    fn test_contains(){
        let s = mat![
            &str:
            ["akasaka","sakanoue","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let b = s.contains("aka");
        let res = mat![
            bool:
            [true,true,true],
            [false,true,true],
            [false,false,false]
        ];
        for i in 0..s.data.len() {
            for j in 0..s.data[i].len() {
                println!("b: {}, res: {}",b.data[i][j], res.data[i][j]);
                assert_eq!(b.data[i][j] == res.data[i][j], true)
            }
        }
    }

    fn test_starts_with(){
        let s = mat![
            &str:
            ["akasaka","sakanoue","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let res = mat![
            bool:
            [false,false,false],
            [false,true,false],
            [false,true,false]
        ];
        assert_eq!(s.starts_with("i") == res, true);
    }

    fn test_ends_with(){
        let s = mat![
            &str:
            ["akasaka","sakanoue","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let res = mat![
            bool:
            [true,false,true],
            [true,true,true],
            [false,true,false]
        ];
        assert_eq!(s.ends_with("i") == res, true);
    }

    fn test_is_empty(){
        let s = mat![
            &str:
            ["she","","is"],
            ["he"," ","does"],
            ["them",".",""]
        ].to_string();
        let res = mat![
            bool:
            [false,true,false],
            [false,false,false],
            [false,false,true]
        ];
        assert_eq!(s.is_empty() == res, true);
    }

    fn test_is_ascii(){
        let s = mat![
            &str:
            ["hirakawa","ﾋﾗｶﾜ","平川"],
            ["122.1","Über",""],
            ["鑛滓","　",","]
        ].to_string();
        let res = mat![
            bool:
            [true,false,false],
            [true,false,true],
            [false,false,true]
        ];
        assert_eq!(s.is_ascii() == res, true);
    }
}