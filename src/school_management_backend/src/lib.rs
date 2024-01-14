#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
// use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

/// Represents information about a student.
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

/// Represents information about a teacher.
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

/// Represents information about a course.
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

/// Represents information about a classroom.
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

/// Represents payload for adding a student.
#[derive(candid::CandidType, Serialize, Deserialize)]
struct StudentPayload {
    name: String,
    grade_level: u8,
    email: String,
    date_of_birth: String,
    address: String,
    guardian_details: String,
    // Additional student-specific payload fields
}

impl Default for StudentPayload {
    fn default() -> Self {
        StudentPayload {
            name: String::default(),
            grade_level: 0,
            email: String::default(),
            date_of_birth: String::default(),
            address: String::default(),
            guardian_details: String::default(),
        }
    }
}

/// Represents payload for adding a teacher.
#[derive(candid::CandidType, Serialize, Deserialize)]
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

impl Default for TeacherPayload {
    fn default() -> Self {
        TeacherPayload {
            name: String::default(),
            subject_area: String::default(),
            email: String::default(),
            qualifications: String::default(),
            employment_date: String::default(),
            address: String::default(),
            assigned_courses: Vec::default(),
            schedule: Vec::default(),
        }
    }
}

/// Represents payload for adding a course.
#[derive(candid::CandidType, Serialize, Deserialize)]
struct CoursePayload {
    name: String,
    description: String,
    teacher_id: u64,
    // Additional course-specific payload fields
}

impl Default for CoursePayload {
    fn default() -> Self {
        CoursePayload {
            name: String::default(),
            description: String::default(),
            teacher_id: 0,
        }
    }
}

/// Represents payload for adding a classroom.
#[derive(candid::CandidType, Serialize, Deserialize)]
struct ClassroomPayload {
    name: String,
    location: String,
    capacity: u32,
    current_course_id: u64,
    // Additional classroom-specific payload fields
}

impl Default for ClassroomPayload {
    fn default() -> Self {
        ClassroomPayload {
            name: String::default(),
            location: String::default(),
            capacity: 0,
            current_course_id: 0,
        }
    }
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

/// Adds a new student with the provided payload.
#[ic_cdk::update]
fn add_student(payload: StudentPayload) -> Result<Student, String> {
    // Validation logic (basic example, add more as needed)
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err("Name and email are required fields".to_string());
    }

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

/// Retrieves information about a student based on the provided ID.
#[ic_cdk::query]
fn get_student(id: u64) -> Result<Student, String> {
    STUDENT_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(student) => Ok(student.clone()),
            None => Err(format!("Student with ID {} not found", id)),
        }
    })
}

/// Updates information about a student based on the provided ID and payload.
#[ic_cdk::update]
fn update_student(id: u64, payload: StudentPayload) -> Result<Student, String> {
    STUDENT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_student) = storage.get(&id) {
            // Clone the existing student to make a mutable copy
            let mut updated_student = existing_student.clone();

            // Update the fields
            updated_student.name = payload.name;
            updated_student.grade_level = payload.grade_level;
            updated_student.email = payload.email;
            updated_student.date_of_birth = payload.date_of_birth;
            updated_student.address = payload.address;
            updated_student.guardian_details = payload.guardian_details;

            // Re-insert the updated student back into the storage
            storage.insert(id, updated_student.clone());

            Ok(updated_student)
        } else {
            Err(format!("Student with ID {} not found", id))
        }
    })
}

/// Deletes a student based on the provided ID.
#[ic_cdk::update]
fn delete_student(id: u64) -> Result<(), String> {
    STUDENT_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Student with ID {} not found", id))
        }
    })
}

/// Adds a new teacher with the provided payload.
#[ic_cdk::update]
fn add_teacher(payload: TeacherPayload) -> Result<Teacher, String> {
    // Validation logic (basic example, add more as needed)
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err("Name and email are required fields".to_string());
    }

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

/// Retrieves information about a teacher based on the provided ID.
#[ic_cdk::query]
fn get_teacher(id: u64) -> Result<Teacher, String> {
    TEACHER_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(teacher) => Ok(teacher.clone()),
            None => Err(format!("Teacher with ID {} not found", id)),
        }
    })
}

/// Updates information about a teacher based on the provided ID and payload.
#[ic_cdk::update]
fn update_teacher(id: u64, payload: TeacherPayload) -> Result<Teacher, String> {
    TEACHER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(existing_teacher) = storage.get(&id) {
            // Clone the existing teacher to make a mutable copy
            let mut updated_teacher = existing_teacher.clone();

            // Update the fields
            updated_teacher.name = payload.name;
            updated_teacher.subject_area = payload.subject_area;
            updated_teacher.assigned_courses = payload.assigned_courses;
            updated_teacher.email = payload.email;
            updated_teacher.qualifications = payload.qualifications;
            updated_teacher.employment_date = payload.employment_date;
            updated_teacher.address = payload.address;
            updated_teacher.schedule = payload.schedule;

            // Re-insert the updated teacher back into the storage
            storage.insert(id, updated_teacher.clone());

            Ok(updated_teacher)
        } else {
            Err(format!("Teacher with ID {} not found", id))
        }
    })
}

/// Deletes a teacher based on the provided ID.
#[ic_cdk::update]
fn delete_teacher(id: u64) -> Result<(), String> {
    TEACHER_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Teacher with ID {} not found", id))
        }
    })
}

/// Adds a new course with the provided payload.
#[ic_cdk::update]
fn add_course(payload: CoursePayload) -> Result<Course, String> {
    // Validation logic (basic example, add more as needed)
    if payload.name.is_empty() || payload.description.is_empty() {
        return Err("Name and description are required fields".to_string());
    }

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

/// Retrieves information about a course based on the provided ID.
#[ic_cdk::query]
fn get_course(id: u64) -> Result<Course, String> {
    COURSE_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(course) => Ok(course.clone()),
            None => Err(format!("Course with ID {} not found", id)),
        }
    })
}

/// Updates information about a course based on the provided ID and payload.
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
            Err(format!("Course with ID {} not found", id))
        }
    })
}

/// Deletes a course based on the provided ID.
#[ic_cdk::update]
fn delete_course(id: u64) -> Result<(), String> {
    COURSE_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Course with ID {} not found", id))
        }
    })
}

/// Adds a new classroom with the provided payload.
#[ic_cdk::update]
fn add_classroom(payload: ClassroomPayload) -> Result<Classroom, String> {
    // Validation logic (basic example, add more as needed)
    if payload.name.is_empty() || payload.location.is_empty() {
        return Err("Name and location are required fields".to_string());
    }

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

/// Retrieves information about a classroom based on the provided ID.
#[ic_cdk::query]
fn get_classroom(id: u64) -> Result<Classroom, String> {
    CLASSROOM_STORAGE.with(|storage| {
        match storage.borrow().get(&id) {
            Some(classroom) => Ok(classroom.clone()),
            None => Err(format!("Classroom with ID {} not found", id)),
        }
    })
}

/// Updates information about a classroom based on the provided ID and payload.
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
            Err(format!("Classroom with ID {} not found", id))
        }
    })
}

/// Deletes a classroom based on the provided ID.
#[ic_cdk::update]
fn delete_classroom(id: u64) -> Result<(), String> {
    CLASSROOM_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Classroom with ID {} not found", id))
        }
    })
}
/// Enrolls a student in a course.
#[ic_cdk::update]
fn enroll_student_in_course(student_id: u64, course_id: u64) -> Result<(), String> {
    // Check if the student and course exist
    let student = get_student(student_id)?;
    let _course = get_course(course_id)?;

    // Check if the student is already enrolled in the course
    if student.enrolled_courses.contains(&course_id) {
        return Err("Student is already enrolled in the course".to_string());
    }

    // Enroll the student in the course
    STUDENT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_student = student.clone();
        updated_student.enrolled_courses.push(course_id);
        storage.insert(student_id, updated_student);
    });

    Ok(())
}
/// Assigns a teacher to a course.
#[ic_cdk::update]
fn assign_teacher_to_course(teacher_id: u64, course_id: u64) -> Result<(), String> {
    // Check if the teacher and course exist
    let teacher = get_teacher(teacher_id)?;
    let course = get_course(course_id)?;

    // Check if the teacher is already assigned to the course
    if course.teacher_id == teacher_id {
        return Err("Teacher is already assigned to the course".to_string());
    }

    // Assign the teacher to the course
    COURSE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_course = course.clone();
        updated_course.teacher_id = teacher_id;
        storage.insert(course_id, updated_course);
    });

    // Update the teacher's assigned courses
    TEACHER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_teacher = teacher.clone();
        updated_teacher.assigned_courses.push(course_id);
        storage.insert(teacher_id, updated_teacher);
    });

    Ok(())
}
/// Updates the equipment in a classroom.
#[ic_cdk::update]
fn update_classroom_equipment(classroom_id: u64, equipment: Vec<String>) -> Result<(), String> {
    // Check if the classroom exists
    let classroom = get_classroom(classroom_id)?;

    // Update the equipment in the classroom
    CLASSROOM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut updated_classroom = classroom.clone();
        updated_classroom.equipment = equipment;
        storage.insert(classroom_id, updated_classroom);
    });

    Ok(())
}


// need this to generate candid
ic_cdk::export_candid!();
