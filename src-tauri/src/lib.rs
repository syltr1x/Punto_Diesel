mod database;
use database::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    id: String,
    item: String,
    price: f32,
    cant: u8,
    tipo: String,
    subtotal: f32,
    iva: f32,
    total: f32
}

#[tauri::command]
fn create_budget(id: &str, date: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, details: Vec<Detail>) {
   insert_budget(id, customer, vehicle, concept, kilometrage, total).unwrap();

   for detail in details.iter() {
       let _ = insert_detail(id, &detail.item, detail.price, detail.cant, &detail.tipo, detail.subtotal, detail.iva, detail.total);
   }
   // Add to balance
   //update_balance(date, "Ingreso", total, &format!("{} - {}", customer, concept)).unwrap();
}
#[tauri::command]
fn create_order(id: &str, paid: f32) -> Result<String, String> {
    let budget = match read_budget(id) {
        Ok(budget) => budget,
        Err(e) => return Err(format!("Error {}. Reading budget with id {}", e, id))
    };
    match insert_order(
        id, &budget.customer, &budget.vehicle, &budget.concept,
        budget.kilometrage, budget.total, paid
    ) {
        Ok(_) => Ok(format!("Order {} created successfully", id)),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
#[tauri::command]
fn create_history(id: &str, pay_date: &str) -> Result<String, String> {
    let order = match read_order(id) {
        Ok(order) => order,
        Err(e) => return Err(format!("Err: ({}) reading order {}", e, id)),
    };
    let res = insert_history(id, order, pay_date)?;
    Ok(res)
}
#[tauri::command]
fn create_customer(name: &str, phone: &str, cuil: &str, dni: &str, tipo: &str, vehicles: Vec<&str>) {
    let _ = database::insert_customer(name, phone, cuil, dni, tipo);
    for vehicle in vehicles.iter() {
        let _ = update_vehicles(vehicle, name);
    }
}
#[tauri::command]
fn create_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) {
    let _ = insert_item(id, name, price, tipo, manufacturer, supplier, model, stock);
}
#[tauri::command]
fn create_worker(name: &str, dni: &str, phone: &str, address: &str, salary: f32) -> Result<String, String> {
    let res = database::insert_worker(name, dni, phone, address, salary)?;
    Ok(res)
}
#[tauri::command]
fn pay_order(id: &str, paid: f32) -> Result<String, String> {
    let res = database::update_order(id, paid)?;
    Ok(res)
}
#[tauri::command]
fn create_payment(name: &str, dni: &str, date: &str, amount: f32) -> Result<String, String> {
    match database::insert_payment(name, dni, date, amount) {
        Ok(_) => "",
        Err(e) => return Err(e),
    };
    let res = database::update_balance(date, "Salida", amount, &format!("Pago a {}", name))?;
    Ok(res)
}

// Read database
#[tauri::command]
fn obtain_budgets() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_budgets()?;
    Ok(res)
}
#[tauri::command]
fn obtain_orders() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_orders()?;
    Ok(res)
}
#[tauri::command]
fn obtain_history() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_history()?;
    Ok(res)
}

#[tauri::command]
fn obtain_details(id: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_details(id)?;
    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_budget, create_customer, create_item, create_order, create_worker, 
            create_payment, create_history, obtain_budgets, obtain_orders, obtain_history, obtain_details, pay_order])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
