pub struct StrLikePy {
    string_data:String,
}
// 次にString型に直にPythonLikeSliceを実装
impl StrLikePy {
    pub fn new(str_data:&str) -> Self {
        Self { string_data:str_data.to_string() }
    }

    // pythonで言うdata[0..5]に近いやり方を定義
    pub fn slice(&self,from:isize,to:isize) -> String {
    
        let mut slice_string = String::from("");
    
        if to < 0 {
            let string_length = (self.string_data.chars().count() as isize + to) as usize;
            let from = from as usize;
            for (i, c )in self.string_data.chars().enumerate(){
                if from <= i && i < string_length {
                    slice_string.push(c);
                }
            }
        } else {
            let to = to as usize;
            let from = from as usize;
            for (i, c )in self.string_data.chars().enumerate(){
                if from <= i && i < to {
                    slice_string.push(c);
                }
            }

        }

        slice_string
    }
}


#[test]
fn string_py_test(){
    let string_data = StrLikePy::new("あいうえお");
    let slice_data = string_data.slice(0, -1);
    println!("{:?}",slice_data);
}
