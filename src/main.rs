use std::{env, io, process};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
struct SettingProgram {
    info_student: bool,
    format_color: Option<String>,
    format_markdown: Option<String>,
    limit: bool,
    start_search: Option<u32>,
    end_search: Option<u32>,
    input_file: Option<String>,
    output_file: Option<String>
}

impl SettingProgram {
    fn new() -> Self {
        Self {
            info_student: false,
            format_color: None,
            format_markdown: Some(String::from("`")),
            limit: false,
            start_search: None,
            end_search: None,
            input_file: None,
            output_file: None,
        }
    }
}

#[derive(Debug)]
struct AllTextCut {
    first_part: String,
    second_part: String,
    third_part: String,
}

impl AllTextCut {
    fn new() -> Self {
        Self {
            first_part: String::new(),
            second_part: String::new(),
            third_part: String::new(),
        }
    }

    fn to_string(&self) -> String {
        format!("{}{}{}", self.first_part, self.second_part, self.third_part)
    }

    fn cutting_text(&mut self, input_text: &mut String, setting_program: &SettingProgram) {
        let lines: Vec<&str> = input_text.lines().collect();
        let len_lines = lines.len();
        let mut lines_editing: (usize, usize) = (0, 0);
        match processing_limit_line(setting_program, len_lines) {
            Ok((s, e)) => lines_editing = (s, e),
            Err(e) => eprintln!("Error: {}", e),
        }
        self.first_part = if lines_editing.0 > 0 {
            lines.iter()
                .take(lines_editing.0 - 1)
                .map(|&line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            String::new()
        };
        self.second_part = lines
            .iter()
            .skip(lines_editing.0 - 1)
            .take(lines_editing.1 - lines_editing.0 + 1)
            .map(|&line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        self.third_part = if lines_editing.1 < len_lines {
            lines.iter()
                .skip(lines_editing.1)
                .take(len_lines - lines_editing.1)
                .map(|&line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            String::new()
        };
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut setting_program: SettingProgram = SettingProgram::new();

    match processing_arguments(&args[1..], &mut setting_program) {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }

    let mut input_string = String::new();

    match read_text(&setting_program) {
        Ok(text) => input_string = text,
        Err(e) => eprintln!("Error: {}", e),
    }

    let mut all_text_cut = AllTextCut::new();
    all_text_cut.cutting_text(&mut input_string, &setting_program);

    editing_text(&mut all_text_cut, &setting_program);

    input_string = all_text_cut.to_string();

    match write_text(&setting_program, &mut input_string) {
        Ok(text) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn processing_limit_line(setting_program: &SettingProgram, len_lines: usize) -> Result<(usize, usize), &'static str> {
    let mut start_search: usize = 0;
    let mut end_search: usize = 0;
    match setting_program.start_search {
        Some(t) => {
            if t as usize > len_lines {
                return Err("Вы пытаетесь передать начало среза, но в тексте меньше строк");
            } else {
                start_search = t as usize;
            }
        },
        None => start_search = 1
    }
    match setting_program.end_search {
        Some(t) => {
            if t as usize > len_lines {
                return Err("Вы пытаетесь передать конец среза, но в тексте меньше строк");
            } else {
                end_search = t as usize;
            }
        },
        None => end_search = len_lines
    }
    Ok((start_search, end_search))
}

fn editing_text(all_text_cut: &mut AllTextCut, setting_program: &SettingProgram) {
    let mut edited_text = &mut all_text_cut.second_part;
    let mut chars: Vec<char> = edited_text.chars().collect();
    let mut i = 0;
    let element: char = '`';
    while i < chars.len() {
        let mut possible_card_number = String::new();
        if chars[i].to_digit(10).is_some() {
            let mut j = i;
            let mut for_dash = 0;
            while j < chars.len() {
                let subsequent_char = chars[j];
                match subsequent_char {
                    '0'..='9' => {
                        possible_card_number.push(subsequent_char);
                        for_dash += 1;
                    },
                    '-' => {
                        if for_dash % 4 == 0 {
                            for_dash = 0;
                        } else {
                            for_dash = 0;
                            break;
                        }
                    }
                    '\n' => {
                        if setting_program.limit {
                            break;
                        }
                    },
                    _ => break,
                }
                if possible_card_number.len() == 16 {
                    if is_valid_card_number(&possible_card_number) {
                        if setting_program.format_markdown.is_some() {
                            chars.insert(i, element);
                            chars.insert(j + 2, element);
                            i += 2;
                        } else if setting_program.format_color.is_some() {
                            chars.splice(i..i, vec!['\x1b', '[', '3', '5', 'm']);
                            chars.splice(j+6..j+6, vec!['\x1b', '[', '0', 'm']);
                            i += 6;
                        }
                    }
                    possible_card_number = String::new();
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    all_text_cut.second_part = chars.iter().collect();
}

fn is_valid_card_number(card_number: &str) -> bool {
    let mut card_vec: Vec<u32> = Vec::new();
    for ch in card_number.chars() {
        if let Some(digit) = ch.to_digit(10) {
            card_vec.push(digit)
        }
    }
    for (index, digit) in card_vec.iter_mut().enumerate() {
        if index % 2 == 0 {
            let doubled = *digit * 2;
            if doubled > 9 {
                *digit = doubled - 9;
            } else {
                *digit = doubled;
            }
        }
    }
    let sum_vec: u32 = card_vec.iter().sum();
    if sum_vec % 10 == 0 {
        true
    } else {
        false
    }
}

fn write_text(setting_program: &SettingProgram, input_text: &mut String) -> Result<(), &'static str> {
    if let Some(ref file_name) = setting_program.output_file {
        let mut file = File::create(file_name).map_err(|_| "Не удалось создать или открыть файл для записи")?;
        file.write_all(&mut input_text.as_bytes()).map_err(|_| "Не удалось записать в файл")?;
    } else {
        println!("{}", input_text);
    }
    Ok(())
}

fn read_text(setting_program: &SettingProgram) -> Result<String, &'static str> {
    let mut input_text = String::new();
    if let Some(ref file_name) = setting_program.input_file {
        let mut file = File::open(file_name).map_err(|_| "Не удалось открыть файл")?;
        file.read_to_string(&mut input_text).map_err(|_| "Не удалось прочитать файл")?;
    } else {
        io::stdin().read_to_string(&mut input_text).map_err(|_| "Не удалось прочитать ввод")?;
    }
    Ok(input_text)
}

fn processing_arguments(args: &[String], setting_program: &mut SettingProgram) -> Result<(), &'static str> {
    let mut keys_end = false;
    for element in args {
        if element.starts_with("-") {
            let key = &element[1..2];
            match key {
                "v" => {
                    if keys_end {
                        return Err("Аргументы нельзя передовать после файлов")

                    }
                    if setting_program.info_student {
                        return Err("В конфигурации программы есть повторяющиеся ключи")
                    }
                    setting_program.info_student = true;
                    print_student_info();
                }
                "c" => {
                    setting_program.format_markdown = None;
                    if keys_end {
                        return Err("Аргументы нельзя передовать после файлов")
                    }
                    match &setting_program.format_color {
                        Some(n) => return Err("В конфигурации программы есть повторяющиеся ключи"),
                        None => {}
                    }
                    setting_program.format_color = Some(String::from("35"));
                }
                "b" => {
                    if keys_end {
                        return Err("Аргументы нельзя передовать после файлов")
                    }
                    match &setting_program.start_search {
                        Some(n) => return Err("В конфигурации программы есть повторяющиеся ключи"),
                        None => {},
                    }
                    match string_to_u32(&element[3..]) {
                        Ok(num) => {
                            match setting_program.end_search {
                                Some(t) => {
                                    if num > t {
                                        return Err("Вы пытаетесь начать поиск с номера строки, который больше номера строки конца поиска")
                                    } else {
                                        setting_program.start_search = Some(num)
                                    }
                                },
                                None => setting_program.start_search = Some(num),
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                "e" => {
                    if keys_end {
                        return Err("Аргументы нельзя передовать после файлов")
                    }
                    match &setting_program.end_search {
                        Some(n) => return Err("В конфигурации программы есть повторяющиеся ключи"),
                        None => {},
                    }
                    match string_to_u32(&element[3..]) {
                        Ok(num) => {
                            match setting_program.start_search {
                                Some(t) => {
                                    if t > num {
                                        return Err("Вы пытаетесь начать поиск с номера строки, который больше номера строки конца поиска")
                                    } else {
                                        setting_program.end_search = Some(num)
                                    }
                                },
                                None => setting_program.end_search = Some(num),
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                "n" => {
                    if keys_end {
                        return Err("Аргументы нельзя передовать после файлов")
                    }
                    if setting_program.limit {
                        return Err("В конфигурации программы есть повторяющиеся ключи")
                    }
                    setting_program.limit = true;
                }
                _ => return Err("Неизвестный ключ")
            }
        } else {
            keys_end = true;
            match check_extesion_file(element) {
                Ok(el) => {}
                 Err(e) => eprintln!("Error: {}", e),
            }
            match &setting_program.input_file {
                Some(T) => {
                    match &setting_program.output_file {
                        Some(T) => return Err("Вы передали больше 2 файлов"),
                        None => setting_program.output_file = Some(element.clone()),
                    }
                },
                None => setting_program.input_file = Some(element.clone()),
            }
        }
    }
    Ok(())
}

fn check_extesion_file(file: &String) -> Result<(), &'static str> {
    if file.len() > 4 {
        if &file[file.len() -4..] != ".txt" {
            return Err("Вы не указали расширение файла")
        }
    } else {
        return Err("Вы не указали расширение файла")
    }
    Ok(())
}

fn print_student_info() {
    println!("Владимир Олегович Баканов, гр. N3145");
    println!("Вариант: 4-1-3-5");
    process::exit(0);
}

fn string_to_u32(string: &str) -> Result<u32, &'static str> {
    if let Ok(number) = string.parse::<u32>() {
        Ok(number)
    } else {
        Err("В аргументы -e и -b передены не числа")
    }
}

