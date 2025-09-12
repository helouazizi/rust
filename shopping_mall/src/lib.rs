pub mod mall;

pub use mall::*;

pub fn biggest_store(mall: &mall::Mall) -> (String, Store) {
    let mut max: u64 = 0;
    let mut storename: String = String::new();
    let mut floorn: String = String::new();

    for (floorname, floor) in mall.floors.clone() {
        for (name, store) in floor.stores {
            if store.square_meters >= max {
                max = store.square_meters;
                storename = name;
                floorn = floorname.clone();
            }
        }
    }
    (
        storename.clone(),
        mall.floors
            .get(&floorn)
            .unwrap()
            .stores
            .get(&storename)
            .unwrap()
            .clone(),
    )
}
pub fn highest_paid_employee(mall: &mall::Mall) -> Vec<(&String, &Employee)> {
    let mut max_salary = 0.0;
    let mut employees_with_max_salary = Vec::new();

    // First pass: find max salary
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (_, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                }
            }
        }
    }

    // Second pass: collect all employees with max salary
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (name, employee) in &store.employees {
                if employee.salary == max_salary {
                    employees_with_max_salary.push((name, employee));
                }
            }
        }
    }

    employees_with_max_salary
}

pub fn nbr_of_employees(mall:&Mall) -> usize {
    let mut nmb_of_employer = mall.guards.len() ;
    for (_, floor) in mall.floors.clone() {
        for (_, store) in floor.stores {
            nmb_of_employer += store.employees.len();
        }
    }

    nmb_of_employer
}
pub fn check_for_securities(mall: &mut Mall, guard: Vec<(String, Guard)>) {
    let mut nmb_of_guard = mall.guards.len();
    let mut Mall_square_meter: u64 = 0;
    for (_,floor) in mall.floors.clone(){
        Mall_square_meter += floor.size_limit ;

    }
  for g in guard {
    if Mall_square_meter  as usize / 200 > nmb_of_guard   {
       
        mall.hire_guard(g.0, g.1);
        nmb_of_guard += 1 ;

    }else {
        return
    }
  }

}
pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (_, employ) in store.employees.iter_mut() {
                if employ.working_hours.1 - employ.working_hours.0 >= 10 {
                    employ.raise( employ.salary * 0.1);
                } else {
                    employ.cut(employ.salary * 0.1);
                }
            }
        }
    }
}
