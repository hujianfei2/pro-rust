#[derive(Debug)]
struct Student {
    name : String,
    age : i32,
    grade : f32,
}
impl Student {
    fn new(&self) -> Student {
        Student{
            name : self.name.clone(),
            age: 0,
            grade: 0.0,
        }
    } 
}
fn main() {
    /*let mut student = Student {
        name : String::from("Jocker"),
        age : 18,
        grade : 0.0,
    };
    student.age = 28;*/
    let student = Student {
        name : String::from("Jocker"),
        age : 18,
        grade : 0.0,
    }.new();
    println!("student : {:#?}",student)
}


