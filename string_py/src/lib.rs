pub struct EasyString {
    string_data:String,
}
// 次にString型に直にPythonLikeSliceを実装
impl EasyString {
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

    pub fn len(&self)->usize{
        self.string_data.chars().count()
    }
}


#[test]
fn string_py_test(){
    let easy_string = EasyString::new("あいうえお");
    let slice_data = easy_string.slice(0, -1);
    assert_eq!("あいうえ",slice_data);
    assert_eq!(5,easy_string.len());
    

}
