use std::{fs::File, io::Read};

use crate::{
    file_parsing::{definitions::Definition, YytextMode},
    lex_creation::YYLEX,
};

use super::{DEFINES, INCLUDES, VARIABLES};

fn open_template_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn write_includes(file: &mut dyn std::io::Write) -> std::io::Result<()> {
    let file_content = open_template_file(INCLUDES)?;
    file.write_all(file_content.as_bytes())?;
    Ok(())
}

pub fn write_defines(
    file: &mut dyn std::io::Write,
    definitions: &[Definition],
    mode: YytextMode,
) -> std::io::Result<()> {
    let file_content = open_template_file(DEFINES)?;
    file.write_all(file_content.as_bytes())?;

    if let YytextMode::Array(n) = mode {
        writeln!(file, "#define YYLMAX {n}")?;
    }

    for elem in definitions {
        match elem {
            Definition::ExclusiveState {
                name: state_name,
                state_nb,
            }
            | Definition::InclusiveState {
                name: state_name,
                state_nb,
            } => {
                writeln!(file, "#define {state_name} {state_nb}")?;
            }
            _ => {}
        }
    }
    writeln!(file)?;
    Ok(())
}

pub fn write_variables(
    definitions: &[Definition],
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let mut file_content = open_template_file(VARIABLES)?;
    let mut to_add = String::new();

    for elem in definitions {
        match elem {
            Definition::Bloc { content } | Definition::LineWithSpace { content } => {
                to_add.push_str(content);
                to_add.push('\n');
            }
            _ => {}
        }
    }

    file_content = file_content.replace("change_me_in_variables!", &to_add);
    file.write_all(file_content.as_bytes())?;
    Ok(())
}

fn write_yytext_pointer(file: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(file, "char *yytext = NULL;")?;
    writeln!(file, "int yyleng = 0;")?;
    writeln!(file)?;
    writeln!(file, "char yy_yytext_last_char(void) {{")?;
    writeln!(file, "\treturn yyleng > 0 ? yytext[yyleng - 1] : 0;")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "void yy_set_yytext(a_elem matching_state) {{")?;
    writeln!(file, "\tif (yymore_flag == 0) {{")?;
    writeln!(file, "\t\tyyleng = matching_state.len_match;")?;
    writeln!(file, "\t\tfree(yytext);")?;
    writeln!(file, "\t\tyytext = malloc(sizeof(char) * (yyleng + 1));")?;
    writeln!(file, "\t\tif (yytext == NULL)")?;
    writeln!(
        file,
        "\t\t\tyy_fatal_error(\"out of dynamic memory in set_yytext()\");"
    )?;
    writeln!(file, "\t\tmemcpy(yytext, buffer.str, yyleng);")?;
    writeln!(file, "\t\tyytext[yyleng] = '\\0';")?;
    writeln!(file, "\t}}")?;
    writeln!(file, "\telse {{")?;
    writeln!(
        file,
        "\t\tyytext = realloc(yytext, yyleng + matching_state.len_match + 1);"
    )?;
    writeln!(file, "\t\tif (yytext == NULL)")?;
    writeln!(
        file,
        "\t\t\tyy_fatal_error(\"out of dynamic memory in set_yytext()\");"
    )?;
    writeln!(
        file,
        "\t\tmemcpy(&yytext[yyleng], buffer.str, matching_state.len_match);"
    )?;
    writeln!(file, "\t\tyyleng += matching_state.len_match;")?;
    writeln!(file, "\t\tyytext[yyleng] = '\\0';")?;
    writeln!(file, "\t\tyymore_flag = 0;")?;
    writeln!(file, "\t}}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "int yyless(int n) {{")?;
    writeln!(file, "\tchar *dest;")?;
    writeln!(file, "\tif (n > yyleng)")?;
    writeln!(
        file,
        "\t\tyy_fatal_error(\"n is bigger than length of yytext!\");"
    )?;
    writeln!(file, "\tdest = malloc(sizeof(char) * (n + 1));")?;
    writeln!(file, "\tif (!dest)")?;
    writeln!(
        file,
        "\t\tyy_fatal_error(\"out of dynamic memory in set_yytext()\");"
    )?;
    writeln!(file, "\tmemcpy(dest, yytext, n);")?;
    writeln!(file, "\tdest[n] = '\\0';")?;
    writeln!(file, "\tfree(yytext);")?;
    writeln!(file, "\tyytext = dest;")?;
    writeln!(file, "\tyyleng = n;")?;
    writeln!(file, "\treturn 1;")?;
    writeln!(file, "}}")
}

fn write_yytext_array(n: usize, file: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(file, "char yytext[{n}];")?;
    writeln!(file, "int yyleng = 0;")?;
    writeln!(file)?;
    writeln!(file, "char yy_yytext_last_char(void) {{")?;
    writeln!(file, "\treturn yyleng > 0 ? yytext[yyleng - 1] : 0;")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "void yy_set_yytext(a_elem matching_state) {{")?;
    writeln!(file, "\tif (yymore_flag == 0) {{")?;
    writeln!(file, "\t\tyyleng = matching_state.len_match;")?;
    writeln!(file, "\t\tmemcpy(yytext, buffer.str, yyleng);")?;
    writeln!(file, "\t\tyytext[yyleng] = '\\0';")?;
    writeln!(file, "\t}}")?;
    writeln!(file, "\telse {{")?;
    writeln!(
        file,
        "\t\tmemcpy(&yytext[yyleng], buffer.str, matching_state.len_match);"
    )?;
    writeln!(file, "\t\tyyleng += matching_state.len_match;")?;
    writeln!(file, "\t\tyytext[yyleng] = '\\0';")?;
    writeln!(file, "\t\tyymore_flag = 0;")?;
    writeln!(file, "\t}}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "int yyless(int n) {{")?;
    writeln!(file, "\tif (n > yyleng)")?;
    writeln!(
        file,
        "\t\tyy_fatal_error(\"n is bigger than length of yytext!\");"
    )?;
    writeln!(file, "\tyytext[n] = '\\0';")?;
    writeln!(file, "\tyyleng = n;")?;
    writeln!(file, "\treturn 1;")?;
    writeln!(file, "}}")
}

pub fn write_yytext_section(
    mode: YytextMode,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    match mode {
        YytextMode::Pointer => write_yytext_pointer(file)?,
        YytextMode::Array(n) => write_yytext_array(n, file)?,
    }
    writeln!(file)?;
    Ok(())
}

pub fn write_user_routine(
    user_routine: &str,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    file.write_all(user_routine.as_bytes())?;
    Ok(())
}

pub fn write_yylex(
    file: &mut dyn std::io::Write,
    in_yylex: &[String],
    mode: YytextMode,
) -> std::io::Result<()> {
    let file_content = open_template_file(YYLEX)?;
    let mut in_yylex_content = String::new();

    for elem in in_yylex {
        in_yylex_content += elem;
    }

    let free_yytext = match mode {
        YytextMode::Pointer => "free(yytext);\n\tyytext = NULL;",
        YytextMode::Array(_) => "",
    };

    let replaced = file_content
        .replace("#write_in_yylex", &in_yylex_content)
        .replace("change_me_free_yytext!", free_yytext);
    file.write_all(replaced.as_bytes())?;
    Ok(())
}
