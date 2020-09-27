use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::env;
use std::process;

extern crate regex;

fn get_args_vector() -> Vec<String> {
  let mut pure_args = env::args();
  let mut args: Vec<String> = Vec::new();

  loop {
    if pure_args.len() == 0 {
      break
    }
    let a: String = pure_args.nth(0).unwrap();
    args.push(a);
  }

  args
}

fn get_all_directories_in_path() -> Result<(Vec<String>, Vec<String>), io::Error> {
  let exec_path = env::current_dir()?;
  
  let entries = fs::read_dir(exec_path)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  let mut dirs: Vec<String> = Vec::new();
  let mut files: Vec<String> = Vec::new();

  for e in entries {
    let path_str = match e.to_str() {
      Some(v) => v,
      None => ""
    };
    if e.is_dir() {
      dirs.push(String::from(path_str));
    } else {
      files.push(String::from(path_str));
    }
  }

  Ok((dirs, files))
}

fn read_file_content(file_name: &String) -> Result<Vec<u8>, io::Error> {
  let mut file = File::open(file_name)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  Ok(buffer)
}

fn xor_file_content(v: &mut Vec<u8>, password: &String) {
  let mut ipassword = 0;
  for i in 0..v.len() {
    let ascii_value = password.chars().nth(ipassword).unwrap() as u8;
    v[i] = v[i] ^ ascii_value;
    ipassword += 1;
    if ipassword == password.len() {
      ipassword = 0;
    }
  }
}

fn write_xored_contend_to_files(v: &mut Vec<u8>, filename: &String) -> io::Result<()> {
  let mut file = File::create(filename)?;
  file.write_all(&v)?;
  Ok(())
}

fn report_error(error_message: &str) {
  print!("Error: {}.\n", error_message);
  process::exit(0);
}

fn main() -> io::Result<()> {
  let args = get_args_vector();
  
  if args.len() < 2 {
    report_error("The program need to receive an password to encript the files");
  }

  if args[1].len() < 12 {
    report_error("The password must be at east 12 characters long");
  }

  let password = args[1].clone();

  let (_, files) = match get_all_directories_in_path() {
    Ok(v) => v,
    Err(v) => return Err(v),
  };

  let re = regex::Regex::new(r"files_are_encripted").unwrap();

  let mut is_decripting = false;
  for filename in files {
    if re.is_match(&filename) {
      is_decripting = true;
      fs::remove_file(&filename)?;
    } else {
      let mut content = match read_file_content(&filename) {
        Ok(v) => v,
        Err(e) => return Err(e),
      };
      xor_file_content(&mut content, &password);
      write_xored_contend_to_files(&mut content, &filename)?;
    }
  }

  if !is_decripting {
    let exec_path = env::current_dir()?;
    if cfg!(windows) {
      File::create(String::from(exec_path.to_str().unwrap()) + "\\files_are_encripted.txt")?;
    } else if cfg!(unix) {
      File::create(String::from(exec_path.to_str().unwrap()) + "/files_are_encripted.txt")?;
    }
  }

  Ok(())
}
