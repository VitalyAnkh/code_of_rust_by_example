use std::fmt;

pub struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"[");
        let v=&self.0;
        for (count,value) in v.iter().enumerate(){
            if count!=0{write!(f,",");}
            write!(f,"{}",value);
        }
        write!(f,"]")
    }
}

#[test]
fn test_list(){
    let l=List(vec![2,434,52,3]);
    println!("{}",l);
}