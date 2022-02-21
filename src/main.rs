fn main() {

}

struct Person {
    first: String,
    last: String
}

enum OrderQuantity {
    UnitQuantity(i32),
    KilogramQuantity(f32)
}

fn print_quantity(quantity: OrderQuantity) -> String{
    match quantity {
        OrderQuantity::UnitQuantity(quantity) => return format!("{} unit", quantity),
        OrderQuantity::KilogramQuantity(quantity) => return format!("{} kg", quantity),
    }
}

#[cfg(test)]
mod tests {
    use crate::{OrderQuantity, Person, print_quantity};

    #[test]
    fn 構造体の定義と分解() {
        let person = Person {
            first: String::from("Alex"),
            last: String::from("Adams")
        };
    
        let Person{first: print_first, last: print_last} = person;
    
        println!("Hello, {first} {last}", first=print_first, last=print_last);

        assert_eq!(print_first, "Alex");
        assert_eq!(print_last, "Adams");
    }

    #[test]
    fn 列挙体とパターンマッチ(){
        assert_eq!(print_quantity(OrderQuantity::UnitQuantity(10)), "10 unit");
        assert_eq!(print_quantity(OrderQuantity::KilogramQuantity(2.5)), "2.5 kg");
    }
}