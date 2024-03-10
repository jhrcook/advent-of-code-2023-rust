use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File aready exists: {}.", .0)]
    FileExists(String),
    #[error("File writing error.")]
    Write(#[from] std::io::Error),
    #[error("Parse integer error.")]
    ParseInt(#[from] std::num::ParseIntError),
}

fn find_latest_day() -> Result<usize, Error> {
    Ok(*fs::read_dir("src/solutions/")
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap())
        .filter(|fname| fname.starts_with("day") & fname.ends_with("rs"))
        .map(|fname| {
            fname
                .replace("day", "")
                .replace(".rs", "")
                .parse::<usize>()
                .map_err(Error::ParseInt)
        })
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .max()
        .unwrap())
}

fn write_text(filename: &str, text: &str) -> Result<(), Error> {
    let path = Path::new(filename);
    if path.exists() {
        log::error!("Trying to write to file that already exists.");
        return Err(Error::FileExists(filename.to_string()));
    }
    let mut output: File = File::create(path)?;
    write!(output, "{}", text).or_else(|e| Err(Error::Write(e)))
}

fn write_solution_file(day: &usize, title: &str) -> Result<(), Error> {
    let text = fs::read_to_string("templates/_template_day.rs")
        .unwrap()
        .replace("TITLE", title)
        .replace("00", format!("{}", day).as_str());

    let file_name = format!("src/solutions/day{:02}.rs", day);
    write_text(&file_name, &text)
}

fn write_test_file(day: &usize) -> Result<(), Error> {
    let text = fs::read_to_string("templates/_template_test.rs")
        .unwrap()
        .replace("00", format!("{}", day).as_str());

    let file_name = format!("tests/test_day{:02}.rs", day);
    write_text(&file_name, &text)
}

fn add_day_to_mod_list(day: &usize) -> Result<(), Error> {
    let fname = "src/solutions/mod.rs";
    let mut file = OpenOptions::new().append(true).open(fname).unwrap();

    let text = format!("pub mod day{:02};", day);
    writeln!(file, "{}", text).or_else(|e| Err(Error::Write(e)))
}

fn add_day_to_lib_map(day: &usize) -> Result<(), Error> {
    let repl_text = "// <-- INSERT NEW DAY HERE -->";
    let new_text = format!(
        "{:02} => Ok(solutions::day{:02}::main(data_dir)),\n{repl_text}",
        day, day
    );
    let fname = "src/lib.rs";
    let text = fs::read_to_string(fname)
        .unwrap()
        .replace(repl_text, &new_text);
    let mut output: File = File::create(fname)?;
    write!(output, "{}", text).or_else(|e| Err(Error::Write(e)))
}

pub fn make_new_day_files(title: &str) -> Result<(), Error> {
    let next_day = find_latest_day()? + 1;
    write_solution_file(&next_day, title)?;
    write_test_file(&next_day)?;
    add_day_to_mod_list(&next_day)?;
    add_day_to_lib_map(&next_day)
}
