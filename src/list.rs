use crate::core::Matrix;
use crate::mat;

impl Matrix<String>  {

    /// 文字列一致判定
    ///
    /// 各元が特定の文字列リテラルを含むかどうかを判定し、結果をMatrix<bool>で返却する。
    ///
    pub fn contains(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].contains(pat))
            }
        }
        res
    }

    /// 先頭文字列判定
    ///
    /// 各元の文字列先頭が特定の文字列リテラルから始まるかどうかを判定し、結果をMatrix<bool>で返却する。
    ///
    pub fn starts_with(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].starts_with(pat))
            }
        }
        res
    }

    /// 終端文字列判定
    ///
    /// 各元の文字列末尾が特定の文字列リテラルで終わるかどうかを判定し、結果をMatrix<bool>で返却する。
    ///
    pub fn ends_with(&self, pat: &str) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].ends_with(pat))
            }
        }
        res
    }

    /// 空文字列判定
    ///
    /// 各元が空文字列であるかどうかを判定し、結果をMatrix<bool>で返却する。
    ///
    pub fn is_empty(&self) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].is_empty())
            }
        }
        res
    }

    /// ASCII文字列判定
    ///
    /// 各元がASCII文字のみを含むかどうかを判定し、結果をMatrix<bool>で返却する。
    ///
    pub fn is_ascii(&self) -> Matrix<bool> {
        let mut res = mat![bool];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].is_ascii())
            }
        }
        res
    }

    /// 1文字除去
    ///
    /// 各元の終端文字を除去し、データ変更後の自身への参照を返却する
    ///
    pub fn pop_char(&mut self) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].pop();
            }
        }
        self
    }

    /// 1文字追加
    ///
    /// 各元に終端文字を追加し、データ変更後の自身への参照を返却する
    ///
    pub fn push_char(&mut self, ch: char) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].push(ch);
            }
        }
        self
    }

    /// 文字列追加
    ///
    /// 各元の末尾に文字列を追加し、データ変更後の自身への参照を返却する
    ///
    pub fn push_str(&mut self, s: &str) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j].push_str(s);
            }
        }
        self
    }

    /// 文字列置換
    ///
    /// 各元に含まれる特定文字列(from)を、指定文字列(to)で置換する。
    ///
    pub fn replace(&mut self, from: &str, to: &str) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = self.data[i][j].replace(from, to);
            }
        }
        self
    }

    /// 文字列長行列の取得
    ///
    /// 各元の文字列の長さを元とする Matrix<usize> を返却する。
    ///
    pub fn to_strlen(&self) -> Matrix<usize> {
        let mut res = mat![usize];
        for i in 0..self.data.len() {
            res.data.push(Vec::new());
            for j in 0..self.data[i].len() {
                res.data[i].push(self.data[i][j].len())
            }
        }
        res
    }

    /// 先頭空白文字除去
    ///
    pub fn trim_start(&mut self) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = String::from(self.data[i][j].trim_start())
            }
        }
        self
    }

    /// 終端空白文字除去
    ///
    pub fn trim_end(&mut self) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = String::from(self.data[i][j].trim_end())
            }
        }
        self
    }

    /// 先頭及び終端空白文字除去
    ///
    pub fn trim(&mut self) -> &mut Self {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = String::from(self.data[i][j].trim());
            }
        }
        self
    }

    /// バイト列変換
    ///
    pub fn as_bytes(&self) -> Matrix<&[u8]>{
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
            ["akasaka","sakamoto","kosaka"],
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

    #[test]
    fn test_starts_with(){
        let s = mat![
            &str:
            ["akasaka","sakamoto","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let res = mat![
            bool:
            [false,false,false],
            [true,true,false],
            [false,true,false]
        ];
        assert_eq!(s.starts_with("i") == res, true);
    }

    #[test]
    fn test_ends_with(){
        let s = mat![
            &str:
            ["akasaka","sakamoto","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let res = mat![
            bool:
            [true,false,true],
            [true,true,true],
            [false,true,false]
        ];
        assert_eq!(s.ends_with("a") == res, true);
    }

    #[test]
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

    #[test]
    fn test_is_ascii(){
        let s = mat![
            &str:
            ["hirakawa","ﾋﾗｶﾜ","平川"],
            ["122.1","Über",""],
            ["鑛滓","　","🐿"]
        ].to_string();
        let res = mat![
            bool:
            [true,false,false],
            [true,false,true],
            [false,false,false]
        ];
        assert_eq!(s.is_ascii() == res, true);
    }

    #[test]
    fn test_pop_push(){
        let mut s = mat![
            &str:
            ["abc", "def"],
            ["ghi", "jkl"]
        ].to_string();
        let res1 = mat![
            &str:
            ["ab", "de"],
            ["gh", "jk"]
        ].to_string();
        let res2 = mat![
            &str:
            ["abc", "dec"],
            ["ghc", "jkc"]
        ].to_string();
        s.pop_char();
        assert_eq!(s == res1, true);
        s.push_char('c');
        assert_eq!(s == res2, true);
    }

    #[test]
    fn test_pop_push_str(){
        let mut s = mat![
            &str:
            ["可燃ごみ", "燃えないごみ"],
            ["資源ごみ", "粗大ごみ"]
        ].to_string();
        let res1 = mat![
            &str:
            ["可燃", "燃えない"],
            ["資源", "粗大"]
        ].to_string();
        let res2 = mat![
            &str:
            ["可燃ツイート", "燃えないツイート"],
            ["資源ツイート", "粗大ツイート"]
        ].to_string();
        assert_eq!(*s.pop_char().pop_char() == res1, true);
        assert_eq!(*s.push_str("ツイート") == res2, true);
    }

    #[test]
    fn test_replace(){
        let mut s = mat![
            &str:
            ["可燃ごみ", "燃えないごみ"],
            ["資源ごみ", "粗大ごみ"]
        ].to_string();
        let res = mat![
            &str:
            ["可燃ツイート", "燃えないツイート"],
            ["資源ツイート", "粗大ツイート"]
        ].to_string();
        s.replace("ごみ", "ツイート");
        assert_eq!(s == res, true);
    }

    #[test]
    fn test_to_strlen(){
        let s = mat![
            &str:
            ["hirakawa","ﾋﾗｶﾜ","平川"],
            ["122.1","Über",""],
            ["鑛滓","　","🍣🐿"]
        ].to_string();
        let res = mat![
            usize:
            [8,12,6],
            [5,5,0],
            [6,3,8]
        ];
        let e = s.to_strlen();
        e.print();
        assert_eq!(e == res, true);
    }

    #[test]
    fn test_trim(){
        let s1 = mat![
            &str:
            [" I love ", " << penguin >> "],
            ["so much ", ". "]
        ];
        let mut s2 = s1.clone().to_string();
        let mut s1 = s1.to_string();

        let res1 = mat![
            &str:
            ["I love ", "<< penguin >> "],
            ["so much ", ". "]
        ].to_string();

        let res2 = mat![
            &str:
            ["I love", "<< penguin >>"],
            ["so much", "."]
        ].to_string();

        s1.trim_start();
        assert_eq!(s1 == res1, true);
        s1.trim_end();
        assert_eq!(s1 == res2, true);
        s2.trim();
        s2.print();
        assert_eq!(s2 == res2, true);
    }

    #[test]
    fn test_as_bytes(){
        let s = mat![
            &str:
            ["akasaka","sakamoto","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        let t = mat![
            &str:
            ["akasaka","sakamoto","kosaka"],
            ["ikasama","isasaka","kuwasaka"],
            ["kawasaki","ishikawa","shikamoto"]
        ].to_string();
        assert_eq!(s.as_bytes() == t.as_bytes(), true);
    }
}
