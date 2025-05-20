fn main() {
    // 变量
    //variable_sha();
    // 数据类型
    //date_type();
    // 表达式
    /*let x = 1;
    let y = || {
        let x= 3;
        x+1
    };
    let y = x + 1;*/
    // 循环
    //fn_loop();
}

fn fn_loop() {
  /*  loop {
        println!("loop!")
    }*/
    let mut a = 1;
    let num = loop {
        if a == 3 { break 10; }
        a += 1;
    };
    println!("num={}",num);
}

fn variable_sha() {
    // 变量和数据类型 加mut是可变，不加表示不可变
    let a = 1;
    let mut b  = 2;
    println!("a={},b={}",a,b);
    b = 4;
    println!("a={},b={}",a,b);
}

fn date_type() {
    // 数组
    let arr: [i32;5] = [1,2,3,4,5];
    for i in &arr {
        println!("arr[{}]={}",i,i);
    }
    // 元组 
    let tup = (1,2.3,true);
    println!("tup.0={},tup.1={},tup.2={}",tup.0,tup.1,tup.2);
}