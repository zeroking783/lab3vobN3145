use std::{env, io, process};
use std::ops::Add;
use std::fs::File;
use std::io::{Read, Write};
use std::ptr::read;

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

fn main() {

    // Читаю аргументы из командной строки
    let args: Vec<String> = env::args().collect();

    //Создаю структуру с базовыми настройками программы
    let mut setting_program: SettingProgram = SettingProgram::new();

    // Запуская функцию, которая обработает мне все аргументы и изменит структуру
    match processing_arguments(&args[1..], &mut setting_program) {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }

    // В этой переменной будет храниться весь текст, в ней же он будет редактироваться и выводится
    let mut input_string = String::new();

    // Здесь происходит обработка откуда стоит читать текст, ну и ошибки связанные с этим здесь тоже обрабатываются
    match read_text(&setting_program) {
        Ok(text) => input_string = text,
        Err(e) => eprintln!("Error: {}", e),
    }


    // Здесь будут функции которая редактирует текст
    //
    //
    //


    //Здесь функция, которая выводит редактированный текст в файл или в stdout
    match write_text(&setting_program, &mut input_string) {
        Ok(text) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}




fn write_text(setting_program: &SettingProgram, input_text: &mut String) -> Result<(), &'static str> {
    let mut output_text = String::new();
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


// Функция, которая обрабатывает аргументы
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

