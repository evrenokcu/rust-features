#[derive(Debug)]
struct Student {
    name: String,
    id: i32,
}
impl Student {
    fn new(name: String, id: i32) -> Self {
        Self { name, id }
    }
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn id(&self) -> i32 {
        self.id
    }
    fn to_ref(&self) -> StudentRef {
        StudentRef::new(self)
    }
}
#[derive(Debug)]
struct StudentList {
    students: Vec<Student>,
}
impl StudentList {
    fn find<'a, T>(&'a self, predicate: T) -> Option<StudentRef<'a>>
    where
        T: Fn(&&Student) -> bool,
    {
        self.students.iter().find(predicate).map(Student::to_ref)
    }

    fn new(students: &[(&str, i32)]) -> Self {
        Self {
            students: students
                .iter()
                .map(|(name, id)| Student::new(String::from(*name), *id))
                .collect(),
        }
    }
    fn find_by_id<'a>(&'a self, id: i32) -> Option<StudentRef<'a>> {
        self.find(|s| s.id == id)
    }
}
#[derive(Debug)]
pub struct StudentRef<'a> {
    student: &'a Student,
}
impl<'a> StudentRef<'a> {
    fn new(student: &'a Student) -> Self {
        Self { student }
    }
}

impl<'a> PartialEq for StudentRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.student.id == other.student.id
    }
}

pub(crate) fn execute() {
    let list = StudentList::new(&[("evren", 1), ("okcu", 2)]);

    let x_1 = &list.find_by_id(1);
    let x_2 = &list.find_by_id(2);
    println!("{:#?}", x_1);
    println!("are equal: {}", x_1 == x_2);
    println!("are not equal: {}", x_1 != x_2);
}
