use std::collections::HashMap;

pub struct Enterprise {
    pub name: String,
    departments: HashMap<String, Vec<String>>,
}

impl Enterprise {
    pub fn create() -> Self {
        let mut ep = Enterprise {
            name: String::from("Acme. Inc."),
            departments: HashMap::<String, Vec<String>>::new(),
        };
        ep.create_department("Engineering");
        ep.create_department("Sales");
        ep.create_department("Payroll");
        ep
    }

    pub fn create_department(&mut self, newdep: &str) {
        self.departments
            .entry(newdep.to_string())
            .or_insert(Vec::<String>::new());
    }

    pub fn hire_person(&mut self, dep: &str, name: &str) -> Result<(), &str> {
        if let Some(dep) = self.departments.get_mut(dep) {
            dep.push(name.to_string());
            return Ok(());
        }
        Err("Department does not exist!")
    }

    pub fn department_size(&self, dep: &str) -> Result<usize, &str> {
        // match self.departments.get(dep) {
        //     Some(department) => {
        //         if department.len() < 1 ...
        //     }
        //     None => {}
        // }
       
        
        if let Some(employees) = self.departments.get(dep) {
            return Ok(employees.len());
        }
        Err("Department does not exist!")
    }

    pub fn get_departments(&self) -> &HashMap<String, Vec<String>> {
        &self.departments
    }

    pub fn get_personell_in_dep(&self, dep: &str) -> Option<&Vec<String>> {
        self.departments.get(dep)
    }

    pub fn remove_person_in_department(&mut self, dep : &str, name : &str) -> Result<(), &str> {
        if let Some(employees) = self.departments.get_mut(dep) {
            if let Some(index) = employees.iter().position(|x| x == name) {
                
                employees.remove(index);
                return Ok(());
            }
        }
        Err("Employee not found!")
    }
}

#[cfg(test)]
mod tests {

    use super::Enterprise;

    #[test]
    fn remove_person_from_existing_dep() {
        let mut ep = Enterprise::create();
        ep.hire_person("Sales", "Alice").unwrap();
        assert!(ep.remove_person_in_department("Sales", "Alice").is_ok());
        assert!(ep.get_personell_in_dep("Sales").unwrap().is_empty());
    }

    #[test]
    fn remove_person_not_in_dep() {
        let mut ep = Enterprise::create();
        ep.hire_person("Sales", "Alice").unwrap();
        assert!(ep.remove_person_in_department("Sales", "Bob").is_err());
        assert_eq!(ep.get_personell_in_dep("Sales").unwrap().len(), 1);
    }

    #[test]
    fn remove_person_from_nonexistent_dep() {
        let mut ep = Enterprise::create();
        assert!(ep.remove_person_in_department("Marketing", "Alice").is_err());
    }

    #[test]
    fn remove_person_from_empty_dep() {
        let mut ep = Enterprise::create();
        assert!(ep.remove_person_in_department("Sales", "Alice").is_err());
    }

    #[test]
    fn create_existing_dep() {
        let mut ep = Enterprise::create();
        ep.create_department("Engineering");
        assert_eq!(ep.get_departments().len(), 3);
    }

    #[test]
    fn create_new_dep() {
        let mut ep = Enterprise::create();
        ep.create_department("Marketing");

        assert_eq!(ep.get_departments().len(), 4);
    }

    #[test]
    fn put_peter_in_wron_dep() {
        let mut ep = Enterprise::create();
        assert!(ep.hire_person("Facility Management", "peter").is_err())
    }

    #[test]
    fn put_peter_in_sales() {
        let mut ep = Enterprise::create();
        assert!(ep.hire_person("Sales", "peter").is_ok());

        if let Some(sales) = ep.get_personell_in_dep("Sales") {   
                 assert_eq!(1, sales.len());
        }
    }
}
