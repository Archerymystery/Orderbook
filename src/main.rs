use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
enum Side {
    Sell,
    Buy,
}
#[derive(Debug, Clone)]
struct Order {
    user_id: i64,
    amount: i64,
    price: i64,
    side: Side,
}
#[derive(Debug, Clone)]
struct BalanceChange {
    user_id: i64,
    value: i64,
    currency: String,
}
#[derive(Debug, Clone)]
struct Orderbook {
    changes: Vec<BalanceChange>,
    sell_orders: BTreeMap<i64, Vec<Order>>,
    buy_orders: BTreeMap<i64, Vec<Order>>,
}
impl Orderbook {
    fn new() -> Self {
        Orderbook {
            changes: Vec::new(),
            sell_orders: BTreeMap::new(),
            buy_orders: BTreeMap::new(),
        }
    }
    fn match_orders(&mut self) {
        while let Some(mut sell_entry) = self.sell_orders.first_entry()
            && let Some(mut buy_entry) = self.buy_orders.last_entry()
        {
            let sell_price = *sell_entry.key();
            let buy_price = *buy_entry.key();
            if buy_price < sell_price {
                break;
            }
            let sell_vec = sell_entry.get_mut();
            let buy_vec = buy_entry.get_mut();
            if let (Some(sell_order), Some(buy_order)) = (sell_vec.first_mut(), buy_vec.first_mut())
            {
                let match_amount = std::cmp::min(sell_order.amount, buy_order.amount);
                self.changes.push(BalanceChange {
                    user_id: sell_order.user_id,
                    value: -match_amount,
                    currency: "UAH".to_string(),
                });
                self.changes.push(BalanceChange {
                    user_id: sell_order.user_id,
                    value: match_amount * sell_price,
                    currency: "USD".to_string(),
                });

                self.changes.push(BalanceChange {
                    user_id: buy_order.user_id,
                    value: match_amount,
                    currency: "UAH".to_string(),
                });
                self.changes.push(BalanceChange {
                    user_id: buy_order.user_id,
                    value: -(match_amount * sell_price),
                    currency: "USD".to_string(),
                });
                sell_order.amount -= match_amount;
                buy_order.amount -= match_amount;

                if sell_order.amount == 0 {
                    sell_vec.remove(0);
                }
                if buy_order.amount == 0 {
                    buy_vec.remove(0);
                }
            }

            if sell_vec.is_empty() {
                sell_entry.remove();
            }
            if buy_vec.is_empty() {
                buy_entry.remove();
            }
        }
    }
    fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Buy => self.buy_orders.entry(order.price).or_default().push(order),
            Side::Sell => self.sell_orders.entry(order.price).or_default().push(order),
        }
        self.match_orders();
    }

    fn print_changes(&mut self) {
        println!("\n--- BALANCE CHANGES ---");

        if self.changes.is_empty() {
            println!("  [No changes]");
        } else {
            println!("  {:<10} {:<12}  Currency", "User ID", "Change");
            println!("  -------------------------------------");

            for change in &self.changes {
                let value_str = format!("{:+}", change.value);
                println!(
                    "  {:<10} {:<12} {}",
                    change.user_id, value_str, change.currency
                );
            }
        }
        println!();
        self.changes.clear();
    }
    fn print(&self) {
        println!("Orderbook");
        println!("\n--- SELL ---");
        if self.sell_orders.is_empty() {
            println!("  [Empty]");
        } else {
            println!("  {:<10} {:>10}", "Price", "Amount");
            for (price, orders) in self.sell_orders.iter().rev() {
                let sum: i64 = orders.iter().map(|o| o.amount).sum();
                println!("  {:<10} {:>10}", format!("${}", price), sum);
            }
        }
        println!("\n--- BUY ---");
        if self.buy_orders.is_empty() {
            println!("  [Empty]");
        } else {
            println!("  {:<10} {:>10}", "Price", "Amount");
            for (price, orders) in self.buy_orders.iter() {
                let sum: i64 = orders.iter().map(|o| o.amount).sum();
                println!("  {:<10} {:>10}", format!("${}", price), sum);
            }
        }
    }
}
fn main() {
    let mut book = Orderbook::new();

    let theme = ColorfulTheme::default();
    loop {
        let selection = Select::with_theme(&theme)
            .with_prompt("Select action")
            .default(0)
            .items(["Crete order", "Print orderbook", "Print changes", "Quit"])
            .interact()
            .unwrap();

        match selection {
            0 => {
                let side = Select::with_theme(&theme)
                    .with_prompt("Select order type")
                    .default(0)
                    .items(["Buy", "Sell"])
                    .interact()
                    .unwrap();
                let user_id: i64 = Input::with_theme(&theme)
                    .with_prompt("Enter user id")
                    .validate_with(|input: &i64| -> Result<(), String> {
                        if *input > 0 {
                            Ok(())
                        } else {
                            Err("User id must be greater than 0".into())
                        }
                    })
                    .interact_text()
                    .unwrap();
                let price: i64 = Input::with_theme(&theme)
                    .with_prompt("Enter price")
                    .validate_with(|input: &i64| -> Result<(), String> {
                        if *input > 0 {
                            Ok(())
                        } else {
                            Err("Price must be greater than 0".into())
                        }
                    })
                    .interact_text()
                    .unwrap();

                let amount: i64 = Input::with_theme(&theme)
                    .with_prompt("Enter amount")
                    .validate_with(|input: &i64| -> Result<(), String> {
                        if *input > 0 {
                            Ok(())
                        } else {
                            Err("Amount must be greater than 0".into())
                        }
                    })
                    .interact_text()
                    .unwrap();

                book.add_order(Order {
                    user_id,
                    amount,
                    price,
                    side: match side {
                        0 => Side::Buy,
                        1 => Side::Sell,
                        _ => panic!("Unknown error"),
                    },
                });
            }
            1 => {
                book.print();
            }
            2 => {
                book.print_changes();
            }
            _ => {
                break;
            }
        }
    }
}
