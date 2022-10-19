use std::collections::HashMap;

use crate::enums::{
    Side,
    OrderType,
};
use crate::messages::{Message, Body};

#[derive(Debug, Clone, Copy)]
pub struct Order {
    msg_type: OrderType,
    reference: u64,
	side: Side,
	quantity: u32,
	pub stock_locate: u16,
	price: u32,
}

impl Order {
    pub fn new(message: &Message, listing: &mut HashMap<u16, String>) -> Self {
        match &message.body {
            Body::AddOrder(add) => {
                listing.entry(add.stock_locate).or_insert_with(|| add.stock.clone());
                Order {
                    msg_type: OrderType::AddOrder,
                    reference: add.reference,
                    side: add.side,
                    quantity: add.shares,
                    stock_locate: add.stock_locate,
                    price: add.price,
                }
            },
            Body::ExecutedWithPriceOrder(exec) => {
                Order {
                    msg_type: OrderType::ExecutedWithPriceOrder,
                    reference: exec.reference,
                    side: Side::Buy, /* unused */
                    quantity: exec.executed_shares,
                    stock_locate: exec.stock_locate,
                    price: exec.price,
                }
            },
            Body::DeleteOrder(del) => {
                Order {
                    msg_type: OrderType::DeleteOrder,
                    reference: del.reference,
                    side: Side::Buy, /* unused */
                    quantity: 0, /* unused */
                    stock_locate: del.stock_locate,
                    price: 0, /* unused */
                }
            },
            Body::ExecutedOrder(exec) => {
                Order {
                    msg_type: OrderType::ExecutedOrder,
                    reference: exec.reference,
                    side: Side::Buy, /* unused */
                    quantity: exec.executed_shares,
                    stock_locate: exec.stock_locate,
                    price: 0, /* unused */
                }
            },
            Body::ReplaceOrder(repl) => {
                Order {
                    msg_type: OrderType::ReplaceOrder,
                    reference: repl.original_reference,
                    side: Side::Buy, /* unused */
                    quantity: repl.shares,
                    stock_locate: repl.stock_locate,
                    price: repl.price,
                }
            },
            Body::CancelOrder(cancel) => {
                Order {
                    msg_type: OrderType::CancelOrder,
                    reference: cancel.reference,
                    side: Side::Buy, /* unused */
                    quantity: cancel.canceled_shares,
                    stock_locate: cancel.stock_locate,
                    price: 0, /* unused */
                }
            },
            Body::StockDirectory(_) => {
                Order {
                    msg_type: OrderType::CancelOrder, /* unused */
                    reference: 0 , /* unused */
                    side: Side::Buy, /* unused */
                    quantity: 0, /* unused */
                    stock_locate: 0, /* unused */
                    price: 0, /* unused */
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct OrderManager {
    orders_per_stock_locate: HashMap<u16, Vec<Order>>,
}

impl OrderManager {
    pub fn new() -> Self {
        OrderManager {
            orders_per_stock_locate: HashMap::new(),
        }
    }

    pub fn execute_order(&mut self, stock_locate: u16, reference: u64, executed_qty: u32, book_manager: &mut BookManager) {
        self.reduce_quantity(stock_locate, reference, executed_qty, book_manager);    
    }

    pub fn reduce_quantity(&mut self, stock_locate: u16, reference: u64, canceled_qty: u32, book_manager: &mut BookManager) {
        if let Some(orders) = self.orders_per_stock_locate.get_mut(&stock_locate) {
            if let Some(index) = orders.iter().position(|&o| o.reference == reference) {
                if orders[index].quantity - canceled_qty == 0 {
                    orders.remove(index);
                } else {
                    orders[index].quantity -= canceled_qty;
                    book_manager.delete_quantity(
                        stock_locate,
                        orders[index].price,
                        canceled_qty,
                        orders[index].side,
                    );
                }
            }
        }
    }

    pub fn add_order(&mut self, order: &Order, book_manager: &mut BookManager) {
        if let Some(orders) = self.orders_per_stock_locate.get_mut(&order.stock_locate) {
           orders.push(*order); 
        } else {
            self.orders_per_stock_locate.insert(order.stock_locate, vec![*order]);
        }

        book_manager.add_quantity(order.stock_locate, order.price, order.quantity, order.side);
    }

    pub fn delete_order(&mut self, stock_locate: u16, reference: u64, book_manager: &mut BookManager) {
        if let Some(orders) = self.orders_per_stock_locate.get_mut(&stock_locate) {
            if let Some(index) = orders.iter().position(|o| reference == o.reference) {
                book_manager.delete_quantity(
                    stock_locate,
                    orders[index].price,
                    orders[index].quantity,
                    orders[index].side,
                );
                orders.remove(index);
            }
        }
    }

    pub fn modify_order(&mut self, stock_locate: u16, old_ref: u64, new_order: &mut Order, book_manager: &mut BookManager) {
        if let Some(orders) = self.orders_per_stock_locate.get_mut(&stock_locate) {
            if let Some(index) = orders.iter().position(|o| old_ref == o.reference) {
                new_order.side = orders[index].side;
                self.delete_order(stock_locate, old_ref, book_manager);
                self.add_order(new_order, book_manager);
            }
        }
    }


    pub fn process(&mut self, order: &Order, book_manager: &mut BookManager) {
        match &order.msg_type {
            OrderType::AddOrder => self.add_order(&order, book_manager),
            OrderType::ExecutedWithPriceOrder => self.execute_order(order.stock_locate, order.reference, order.quantity , book_manager),
            OrderType::DeleteOrder => self.delete_order(order.stock_locate, order.reference, book_manager),
            OrderType::ExecutedOrder => self.execute_order(order.stock_locate, order.reference, order.quantity , book_manager),
            OrderType::CancelOrder => self.reduce_quantity(order.stock_locate, order.reference, order.quantity, book_manager),
            OrderType::DeleteOrder => self.delete_order(order.stock_locate, order.reference, book_manager),
            OrderType::ReplaceOrder => {
                let mut new_order = Order {
                    msg_type: order.msg_type,
                    reference: order.reference,
                    side: order.side,
                    quantity: order.quantity,
                    stock_locate: order.stock_locate,
                    price: order.price,
                };
                self.modify_order(order.stock_locate, order.reference, &mut new_order, book_manager);
            },
            _ => println!("Not impl yet."),
        }
    }

}

#[derive(Debug)]
pub struct BookManager {
    pub books_per_stock_locate: HashMap<u16, (HashMap<u32, u32>, HashMap<u32, u32>)>,
}

impl BookManager {
    pub fn new() -> Self {
        BookManager {
            books_per_stock_locate: HashMap::new(),
        }
    }

    pub fn add_quantity(&mut self, stock_locate: u16, price: u32, qty: u32, side: Side) {
        if let Some(book) = self.books_per_stock_locate.get_mut(&stock_locate) {
            match side {
                Side::Buy => {
                    if let Some(book_buy_entry) = book.0.get_mut(&price) {
                        *book_buy_entry += qty; 
                    } else {
                        book.0.insert(price, qty); 
                    }
                },
                Side::Sell => {
                    if let Some(book_sell_entry) = book.1.get_mut(&price) {
                        *book_sell_entry += qty; 
                    } else {
                        book.1.insert(price, qty); 
                    }

                },
            }
        } else {
            self.books_per_stock_locate.insert(stock_locate, (HashMap::new(), HashMap::new()));
            self.add_quantity(stock_locate, price, qty, side);
        }
    }

    pub fn delete_quantity(&mut self, stock_locate: u16, price: u32, qty: u32, side: Side) {
        if let Some(book) = self.books_per_stock_locate.get_mut(&stock_locate) {
            match side {
                Side::Buy => {
                    if let Some(book_buy_entry) = book.0.get_mut(&price) {
                        if *book_buy_entry <= qty {
                            book.0.remove(&price); 
                        } else {
                            *book_buy_entry -= qty;
                        }
                    }
                },
                Side::Sell => {
                    if let Some(book_sell_entry) = book.1.get_mut(&price) {
                        if *book_sell_entry <= qty {
                            book.1.remove(&price); 
                        } else {
                            *book_sell_entry -= qty;
                        }
                    }
                }
            }
        }
    }

    pub fn display_book(self, stock_locate: u16, book_depth: usize) {
        if let Some(book) = self.books_per_stock_locate.get(&stock_locate) {
            let mut sell_hash_vec: Vec::<(&u32, &u32)> =  book.1.iter().collect();
            let mut buy_hash_vec: Vec::<(&u32, &u32)> =  book.0.iter().collect();

            sell_hash_vec.sort_by(|a, b| b.0.cmp(a.0));
            buy_hash_vec.sort_by(|a, b| b.0.cmp(a.0));

            println!("----- Sell -----");
            for (level, entry) in sell_hash_vec.iter().rev().enumerate() {
                if level >= book_depth {
                    break;
                }
                println!("[{}] {} @ {:.4}", level, entry.1, *entry.0 as f64 / 10000.0);
            }

            println!("----- Buy -----");
            for (level, entry) in buy_hash_vec.iter().enumerate() {
                if level >= book_depth {
                    break;
                }
                println!("[{}] {} @ {:.4}", level, entry.1, *entry.0 as f64 / 10000.0);
            }

        }
    }

}
