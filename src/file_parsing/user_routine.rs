use super::FileInfo;

pub fn parse_user_routine_part(file: &mut FileInfo) -> String {
    let mut user_routine = String::new();

    while let Some(char) = file.it.next() {
        user_routine.push(char);
    }
    return user_routine;
}
