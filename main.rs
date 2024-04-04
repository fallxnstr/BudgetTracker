use std::error::Error;
use std::fs::File;
use std::sync::{Arc, Mutex};
use csv::Reader;
use csv::Writer;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    // let (total_balance, income, expense) = unwrapcsv(&ui);
    // println!("{:?}", unwrapcsv(&ui));
    //
    // let total_balance = total_balance.parse::<i32>().unwrap();

    let ui_handle = ui.as_weak();
    //Arcs for balance cloning and not breaking
    let total_balance = Arc::new(Mutex::new(0));
    let total_balance_clone = Arc::clone(&total_balance);



    ui.on_add_income(move |string| {
        let ui = ui_handle.unwrap();
        let mut balance = total_balance_clone.lock().unwrap();

        let num: i32 = string.trim().parse().unwrap();
        let income = format!("{}", num);

        // Balance
        *balance += num;
        let balance_result = format!("{}", *balance);

        println!("{}, {}", balance_result, income);

        ui.set_incomeResults((&income).into());
        ui.set_balanceResults((&balance_result).into());

        write(&balance_result, Some(&income), Some(&String::from("0"))).expect("KABooom, BOmba");


    });

    let ui_handle = ui.as_weak();
    let total_balance_clone = Arc::clone(&total_balance);

    ui.on_subtract_income(move |string| {
        let ui = ui_handle.unwrap();
        let mut balance = total_balance_clone.lock().unwrap();

        let num: i32 = string.trim().parse().unwrap();
        let expense = format!("{}", num);

        // Balance
        *balance -= num;
        let balance_result = format!("{}", *balance);



        ui.set_expenseResults((&expense).into());
        ui.set_balanceResults((&balance_result).into());

        println!("{}, {}", balance_result, expense);

        write(&balance_result, Some(&String::from("0")), Some(&expense)).expect("KABooom, BOmba");

    });

    unwrapcsv(&ui);
    ui.run()
}


fn unwrapcsv(uitwoost: &AppWindow) -> (&str, &str, &str) {
    let file_name = "data.csv";
    let result = Reader::from_path(file_name);

    let balance_result = "";
    let income = "";
    let expense = "";

    if result.is_err() {
        File::create("data.csv");
        println!("Error opening CSV, creating a file instead..");
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.records() {
        let number = record.unwrap();

        let balance_result = number.get(0).unwrap();
        let income = number.get(1).unwrap();
        let expense = number.get(2).unwrap();

        // println!("{}, {}, {}", balance_result, income, expense);

        uitwoost.set_incomeResults((income).into());
        uitwoost.set_expenseResults((expense).into());
        uitwoost.set_balanceResults((balance_result).into());


    }
    (&balance_result, &income, &expense)
}

fn write(balance_result: &String, income: Option<&String>, expense: Option<&String>) -> Result<(), Box<dyn Error>>{
    let mut wtr = Writer::from_path("data.csv")?;

    match income {
        Some(_) => {}
        None => panic!("Eduard 8 ashibok")
    }

    match expense {
        Some(_) => {}
        None => panic!("Eduard 10 ashibok")
    }


    let _ = wtr.write_record(&["balance_result", "income", "expense"]);
    let _ = wtr.write_record(&[balance_result, income.unwrap(), expense.unwrap()]);
    wtr.flush()?;
    Ok(())
}
