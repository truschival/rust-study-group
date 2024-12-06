use hireandfire::Enterprise;

fn main() {
    let mut myshop = Enterprise::create();

    myshop.name = "Foobar Inc.".to_string();

    myshop.create_department("C-Suite");
    _ = myshop.hire_person("Maintenance", "Gunnar");
    _ = myshop.hire_person("Engineering", "Dick");
    _ = myshop.hire_person("Engineering", "Peter");
    _ = myshop.hire_person("Sales", "Herbert");

    for (dep, ppl) in myshop.get_departments() {
        
        println!("{:?}",ppl.get(0));
        
        if ppl.is_empty() {
            println!("Apparently nobody works in {}, we have to hire!", dep)
        }
        for employee in ppl {
            println!("{} works in {}", employee, dep);
        }
    }
}
