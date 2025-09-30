#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
    gpa: f64
}

impl Student {

    fn new(name: String, id: u32) -> Self {  // Works as a constructor
        Self { name, id, gpa: 0.0 }
    } 

    fn say_hi(&self) {
        println!("{} says hi!", self.name);
    }

    fn get_gpa(&self) -> f64 {
        self.gpa
    }

    fn set_gpa(&mut self, new_gpa: f64) {
        if new_gpa >= 0.0 && new_gpa <= 4.0 {
            self.gpa = new_gpa;
        }
    }
}

fn main() {
    let mut s1 = Student::new("Juan".to_string(), 123);
    println!("{:?}", s1);
    s1.set_gpa(3.7);
    println!("nombre: {}, matricula: {}, promedio: {}", s1.name, s1.id, s1.gpa);
    s1.say_hi();
    let mut s2 = Student::new("Maria".to_string(), 199);
    s2.say_hi();
    println!("{}", s1.get_gpa());
    s2.set_gpa(4.0);
    println!("{:?}", s2);
}
