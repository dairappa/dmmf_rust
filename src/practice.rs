pub(crate) struct Person {
    pub(crate) first: String,
    pub(crate) last: String
}

pub(crate) enum OrderQuantity {
    UnitQuantity(i32),
    KilogramQuantity(f32)
}

pub(crate) fn print_quantity(quantity: OrderQuantity) -> String{
    match quantity {
        OrderQuantity::UnitQuantity(quantity) => return format!("{} unit", quantity),
        OrderQuantity::KilogramQuantity(quantity) => return format!("{} kg", quantity),
    }
}
