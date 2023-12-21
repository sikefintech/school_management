#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Student {
    id: u64,
    name: String,
    grade_level: u8,
    enrolled_courses: Vec<u64>,
    email: String,
    date_of_birth: String, // Format: YYYY-MM-DD
    address: String,
    guardian_details: String,
    performance_records: Vec<u64>, // IDs of performance/grade records
    attendance_records: Vec<u64>, // IDs of attendance records
    // Additional student-specific fields
}


impl Storable for Student {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}


impl BoundedStorable for Student {
    const MAX_SIZE: u32 = 2048; // Adjust based on the expected size of the struct
    const IS_FIXED_SIZE: bool = false;
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Teacher {
    id: u64,
    name: String,
    subject_area: String,
    assigned_courses: Vec<u64>,
    email: String,
    qualifications: String,
    employment_date: String, // Format: YYYY-MM-DD
    address: String,
    schedule: Vec<u64>, // IDs of scheduled classes or duties
    // Additional teacher-specific fields
}


impl Storable for Teacher {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Teacher {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Course {
    id: u64,
    name: String,
    description: String,
    teacher_id: u64,
    student_ids: Vec<u64>,
    schedule: String, // Course schedule details
    syllabus: String,
    course_materials: Vec<String>, // URLs or IDs of course materials
    // Additional course-specific fields
}


impl Storable for Course {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Course {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Classroom {
    id: u64,
    name: String,
    location: String,
    capacity: u32,
    current_course_id: u64,
    equipment: Vec<String>, // List of classroom equipment/resources
    // Additional classroom-specific fields
}
impl Storable for Classroom {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Classroom {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct StudentPayload {
    name: String,
    grade_level: u8,
    email: String,
    date_of_birth: String,
    address: String,
    guardian_details: String,
    // Additional student-specific payload fields
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TeacherPayload {
    name: String,
    subject_area: String,
    email: String,
    qualifications: String,
    employment_date: String,
    address: String,
    assigned_courses: Vec<u64>, // Newly added field
    schedule: Vec<u64>, // Newly added field
    // Additional teacher-specific payload fields
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CoursePayload {
    name: String,
    description: String,
    teacher_id: u64,
    // Additional course-specific payload fields
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ClassroomPayload {
    name: String,
    location: String,
    capacity: u32,
    current_course_id: u64,
    // Additional classroom-specific payload fields
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STUDENT_STORAGE: RefCell<StableBTreeMap<u64, Student, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TEACHER_STORAGE: RefCell<StableBTreeMap<u64, Teacher, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static COURSE_STORAGE: RefCell<StableBTreeMap<u64, Course, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static CLASSROOM_STORAGE: RefCell<StableBTreeMap<u64, Classroom, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[ic_cdk::update]
fn add_student(payload: StudentPayload) -> Result<Student, String> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let student = Student {
        id,
        name: payload.name,
        grade_level: payload.grade_level,
        enrolled_courses: Vec::new(),
        email: payload.email,
        date_of_birth: payload.date_of_birth,
        address: payload.address,
        guardian_details: payload.guardian_details,
        performance_records: Vec::new(),
        attendance_records: Vec::new(),
    };

    STUDENT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, student.clone());
    });

    Ok(student)
}
#[ic_cdk::query]
fn get_student(id: u64) -> Result<Student, String> {
    STUDENT_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(student) => Ok(student.clone()),
            None => Err("Student not found".to_string()),
        }
    })
}


#[ic_cdk::update]
fn update_student(id: u64, payload: StudentPayload) -> Result<Student, String> {
    STUDENT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_student) = storage.get(&id) {
            // Clone the existing student to make a mutable copy
            let mut updated_student = existing_student.clone();

            // Update the fields
            updated_student.name = payload.name;
            // Update other fields similarly...

            // Re-insert the updated student back into the storage
            storage.insert(id, updated_student.clone());

            Ok(updated_student)
        } else {
            Err("Student not found".to_string())
        }
    })
}

#[ic_cdk::update]
fn delete_student(id: u64) -> Result<(), String> {
    STUDENT_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Student not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn add_teacher(payload: TeacherPayload) -> Result<Teacher, String> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let teacher = Teacher {
        id,
        name: payload.name,
        subject_area: payload.subject_area,
        assigned_courses: Vec::new(),
        email: payload.email,
        qualifications: payload.qualifications,
        employment_date: payload.employment_date,
        address: payload.address,
        schedule: Vec::new(),
    };

    TEACHER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, teacher.clone());
    });

    Ok(teacher)
}
#[ic_cdk::query]
fn get_teacher(id: u64) -> Result<Teacher, String> {
    TEACHER_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(teacher) => Ok(teacher.clone()),
            None => Err("Teacher not found".to_string()),
        }
    })
}
#[ic_cdk::update]
fn update_teacher(id: u64, payload: TeacherPayload) -> Result<Teacher, String> {
    TEACHER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_teacher) = storage.get(&id) {
            let mut updated_teacher = existing_teacher.clone();

            updated_teacher.name = payload.name;
            updated_teacher.subject_area = payload.subject_area;
            updated_teacher.assigned_courses = payload.assigned_courses;
            updated_teacher.email = payload.email;
            updated_teacher.qualifications = payload.qualifications;
            updated_teacher.employment_date = payload.employment_date;
            updated_teacher.address = payload.address;
            updated_teacher.schedule = payload.schedule;

            storage.insert(id, updated_teacher.clone());

            Ok(updated_teacher)
        } else {
            Err("Teacher not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn delete_teacher(id: u64) -> Result<(), String> {
    TEACHER_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Teacher not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn add_course(payload: CoursePayload) -> Result<Course, String> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let course = Course {
        id,
        name: payload.name,
        description: payload.description,
        teacher_id: payload.teacher_id,
        student_ids: Vec::new(),
        schedule: String::new(), // Initial empty schedule
        syllabus: String::new(), // Initial empty syllabus
        course_materials: Vec::new(), // Initial empty course materials
    };

    COURSE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, course.clone());
    });

    Ok(course)
}
#[ic_cdk::query]
fn get_course(id: u64) -> Result<Course, String> {
    COURSE_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(course) => Ok(course.clone()),
            None => Err("Course not found".to_string()),
        }
    })
}
#[ic_cdk::update]
fn update_course(id: u64, payload: CoursePayload) -> Result<Course, String> {
    COURSE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_course) = storage.get(&id) {
            let mut updated_course = existing_course.clone();
            updated_course.name = payload.name;
            updated_course.description = payload.description;
            updated_course.teacher_id = payload.teacher_id;
            // Note: Student IDs, schedule, syllabus, and materials are not updated here
            storage.insert(id, updated_course.clone());
            Ok(updated_course)
        } else {
            Err("Course not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn delete_course(id: u64) -> Result<(), String> {
    COURSE_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Course not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn add_classroom(payload: ClassroomPayload) -> Result<Classroom, String> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let classroom = Classroom {
        id,
        name: payload.name,
        location: payload.location,
        capacity: payload.capacity,
        current_course_id: payload.current_course_id,
        equipment: Vec::new(), // Initial empty equipment list
    };

    CLASSROOM_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, classroom.clone());
    });

    Ok(classroom)
}
#[ic_cdk::query]
fn get_classroom(id: u64) -> Result<Classroom, String> {
    CLASSROOM_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(classroom) => Ok(classroom.clone()),
            None => Err("Classroom not found".to_string()),
        }
    })
}
#[ic_cdk::update]
fn update_classroom(id: u64, payload: ClassroomPayload) -> Result<Classroom, String> {
    CLASSROOM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_classroom) = storage.get(&id) {
            let mut updated_classroom = existing_classroom.clone();
            updated_classroom.name = payload.name;
            updated_classroom.location = payload.location;
            updated_classroom.capacity = payload.capacity;
            updated_classroom.current_course_id = payload.current_course_id;
            // Equipment is not updated here
            storage.insert(id, updated_classroom.clone());
            Ok(updated_classroom)
        } else {
            Err("Classroom not found".to_string())
        }
    })
}
#[ic_cdk::update]
fn delete_classroom(id: u64) -> Result<(), String> {
    CLASSROOM_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Classroom not found".to_string())
        }
    })
}


// need this to generate candid
ic_cdk::export_candid!();
