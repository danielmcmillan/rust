use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

struct Company {
  departments: HashMap<String, HashSet<String>>,
}

impl Company {
  fn new() -> Company {
    Company {
      departments: HashMap::new(),
    }
  }

  fn add_name(self: &mut Company, department: &str, name: &str) {
    let names = self
      .departments
      .entry(String::from(department))
      .or_insert(HashSet::new());
    names.insert(String::from(name));
  }

  fn list_names(&self) {
    let mut departments: Vec<&String> = self.departments.keys().collect();
    departments.sort_unstable();
    for department in &departments {
      self.list_names_by_department(department);
    }
    if departments.is_empty() {
      println!("No departments");
    }
  }

  fn list_names_by_department(&self, department: &str) {
    println!("{}", department);
    let names = self.departments.get(department);
    if let Some(names) = names {
      let mut names: Vec<&String> = names.iter().collect();
      names.sort_unstable();

      for name in names {
        println!(" - {}", name);
      }
    } else {
      println!("No employees");
    }
  }
}

pub fn start() {
  let mut company = Company::new();

  loop {
    println!("Add <user> to <department>, or List <department>, or List");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match &parts[..] {
      &["add", name, "to", department] => company.add_name(department, name),
      &["list", department] => company.list_names_by_department(department),
      &["list"] => company.list_names(),
      &["quit"] => break,
      &[] => continue,
      _ => println!("Invalid command"),
    }
  }
}
