use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Directory {
    departments: HashMap<String, Vec<String>>,
    homes: HashMap<String, String>,
}

fn join_words(words: &[&str]) -> String {
    let mut out = String::new();
    for word in words {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

fn list_line(department: &str, names: &[String]) -> String {
    let mut line = format!("{department}: ");
    if names.is_empty() {
        line.push_str("(none)");
    }
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            line.push_str(", ");
        }
        line.push_str(name);
    }
    line.push('\n');
    line
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            departments: HashMap::new(),
            homes: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, department: &str) -> bool {
        let name = name.trim();
        let department = department.trim();
        if name.is_empty() || department.is_empty() {
            return false;
        }
        if let Some(old) = self.homes.get(name) {
            if old == department {
                return false;
            }
            let old = old.clone();
            self.take_out(name, &old);
        }
        self.departments
            .entry(department.to_string())
            .or_default()
            .push(name.to_string());
        self.homes.insert(name.to_string(), department.to_string());
        true
    }

    fn take_out(&mut self, name: &str, department: &str) {
        let mut now_empty = false;
        if let Some(names) = self.departments.get_mut(department) {
            let mut i = 0;
            while i < names.len() {
                if names[i] == name {
                    names.remove(i);
                    break;
                }
                i += 1;
            }
            now_empty = names.is_empty();
        }
        if now_empty {
            self.departments.remove(department);
        }
    }

    pub fn department(&self, name: &str) -> Vec<String> {
        let mut names = match self.departments.get(name.trim()) {
            Some(names) => names.clone(),
            None => Vec::new(),
        };
        names.sort();
        names
    }

    pub fn all(&self) -> Vec<(String, Vec<String>)> {
        let mut keys = Vec::new();
        for (key, _) in &self.departments {
            keys.push(key.clone());
        }
        keys.sort();
        let mut out = Vec::new();
        for key in keys {
            let names = self.department(&key);
            out.push((key, names));
        }
        out
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let name = name.trim();
        match self.homes.remove(name) {
            Some(department) => {
                self.take_out(name, &department);
                true
            }
            None => false,
        }
    }

    pub fn command(&mut self, line: &str) -> String {
        let mut words = Vec::new();
        for word in line.split_whitespace() {
            words.push(word);
        }
        if words.is_empty() {
            return String::from("Unknown command\n");
        }
        let rest = &words[1..];
        match words[0] {
            "Add" => self.add_command(rest),
            "List" => {
                if rest.is_empty() {
                    return String::from("Usage: List <department>\n");
                }
                let department = join_words(rest);
                list_line(&department, &self.department(&department))
            }
            "Remove" => {
                if rest.is_empty() {
                    return String::from("Usage: Remove <name>\n");
                }
                let name = join_words(rest);
                if self.remove(&name) {
                    format!("Removed {name}\n")
                } else {
                    format!("{name} is not in the directory\n")
                }
            }
            "All" => {
                if !rest.is_empty() {
                    return String::from("Usage: All\n");
                }
                let groups = self.all();
                if groups.is_empty() {
                    return String::from("(no employees)\n");
                }
                let mut out = String::new();
                for (department, names) in &groups {
                    out.push_str(&list_line(department, names));
                }
                out
            }
            _ => String::from("Unknown command\n"),
        }
    }

    fn add_command(&mut self, rest: &[&str]) -> String {
        let mut to = None;
        for (i, word) in rest.iter().enumerate() {
            if *word == "to" {
                to = Some(i);
                break;
            }
        }
        let Some(k) = to else {
            return String::from("Usage: Add <name> to <department>\n");
        };
        if k == 0 || k + 1 == rest.len() {
            return String::from("Usage: Add <name> to <department>\n");
        }
        let name = join_words(&rest[..k]);
        let department = join_words(&rest[k + 1..]);
        let reply = match self.homes.get(&name) {
            Some(old) => {
                if *old == department {
                    return format!("{name} is already in {department}\n");
                }
                format!("Moved {name} to {department}\n")
            }
            None => format!("Added {name} to {department}\n"),
        };
        self.add(&name, &department);
        reply
    }
}
