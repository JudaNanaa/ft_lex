use super::FileInfo;

pub fn parse_user_routine_part(file: &mut FileInfo) -> String {
    let mut user_routine = String::new();

    for char in file.it.by_ref() {
        user_routine.push(char);
    }
    return user_routine;
}
