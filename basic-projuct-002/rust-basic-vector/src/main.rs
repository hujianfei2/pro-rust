use crate::DataType::{Flow, Int, Text};
fn main() {
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    let mut row = vec![
        Flow(12.5),
        Int(1),
        Text(String::from("Hello World!")),
    ];
    row.push(Text(String::from("Hello Baby!")));
    for i in &row {
        match i {
            Int(i) => println!("{}",i),
            Text(s) => println!("{}",s),
            Flow(f) => println!("{}",f),
        };
    }
    
    let mut row2 = vec![1,2,4];  // 它具有内存地址的所有权
    let v = &row2[0]; // 这里取的是row2这个变量在栈内存里面的地址，而非row2这个变量执行的堆内存的地址，所以她拿不走row2变量对堆内存指向的所有权
    println!("{}",v); 
    row2.push(5);// 这里相当于给堆内存中的数据变化了，地址也变了，你前面已经取到的值所指向的地址就是存在了。因此当堆内存改变之后，前面变量拿到的值就是无效的。
    // println!("{}",row2);
    for i in row2.iter() {
        match i {  
            1 => println!("{}",i),
            _ => {
                println!("non");
            },
        };
    }
}

enum DataType {
    Flow(f64),
    Int(i32),
    Text(String),
}
