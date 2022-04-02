trait Person{
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ComSciStudent: Programmer + Student {
    fn git_username(&self) -> String; 
}
fn com_scip_student_greting(student: &dyn ComSciStudent) -> String {
    format!(
        "My Name is {}, and I attend {}. My favourite language is {}, My git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
        )
}

fn main(){}
