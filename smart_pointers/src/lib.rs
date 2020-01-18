pub mod cons_list;
pub use cons_list::List;

pub fn run() {
  let list = List::new(&[1, 2, 3]);
  for item in list {
    println!("List item: {}", item);
  }
}
