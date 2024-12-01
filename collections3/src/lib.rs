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

    pub fn hire_person(&mut self, dep: &str, name: &str) -> Result<bool, &str> {
        if let Some(dep) = self.departments.get_mut(dep) {
            dep.push(name.to_string());
            return Ok(true);
        }
        Err("Department does not exist!")
    }

    pub fn department_size(&self, dep: &str) -> Result<usize, &str> {
        if let Some(dep) = self.departments.get(dep) {
            return Ok(dep.len());
        }
        Err("Department does not exist!")
    }

    pub fn get_departments(&self) -> &HashMap<String, Vec<String>> {
        &self.departments
    }

    pub fn get_personell_in_dep(&self, dep: &str) -> Option<&Vec<String>> {
        self.departments.get(dep)
    }
}

#[cfg(test)]
mod tests {

    use super::Enterprise;

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
