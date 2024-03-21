pub struct StrLikePy {
    string_data:String,
}
// 次にString型に直にPythonLikeSliceを実装
impl StrLikePy {
    pub fn new(str_data:&str) -> Self {
        Self { string_data:str_data.to_string() }
    }

    // pythonで言うdata[0..5]に近いやり方を定義
    pub fn slice(&self,from:usize,to:usize) -> String {
    
        let mut slice_string = String::from("");
    
        for (i, c )in self.string_data.chars().enumerate(){
            if from <= i && i < to {
                slice_string.push(c);
            }
        }

        slice_string
    }
}
