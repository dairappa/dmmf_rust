fn main() {}

mod ordertracking_domain {

    use regex::Regex;
    use uom::si::mass::kilogram;
    pub struct WidgetCode(String);
    #[derive(Debug)]
    pub enum WidgetError {
        CreationError,
    }
    impl WidgetCode {
        pub fn create(code: &str) -> Result<WidgetCode, WidgetError> {
            let re = Regex::new(r"^W\d{4}").unwrap();
            if !re.is_match(code){
                return Err(WidgetError::CreationError);
            }

            Ok(WidgetCode(String::from(code)))
        }

        pub fn value(&self) -> String{
            self.0.clone()
        }
    }
    struct GizmoCode(String);
    enum ProductCode{
        Widget(WidgetCode),
        Gizmo(GizmoCode)
    }

    pub enum QuantityError {
        CreationError
    }
    struct KilogramQuantity(kilogram);

    pub struct UnitQuantity(i32);
    impl UnitQuantity {
        pub fn create(quantity: i32) -> Result<UnitQuantity, QuantityError>{
            if quantity < 1 {
                return Err(QuantityError::CreationError);
            } else if quantity > 1000 {
                return Err(QuantityError::CreationError);
            } else {
                return Ok(UnitQuantity(quantity));
            }
        }
    }

    enum OrderQuantity {
        KilogramQuantity(KilogramQuantity),
        UnitQuantity(UnitQuantity),
    }

    struct OrderId();
    struct OrderLineId();
    struct CustomerId();

    struct CustomerInfo();

    struct ShippingAddress();
    struct BillingAddress();
    struct Price();
    struct BilingAmount();

    struct Order {
        id: OrderId,
        customer_id: CustomerId,
        shipping_address: ShippingAddress,
        billing_address: BillingAddress,
        orderlines: Vec<OrderLine>,
        amount_to_bill: BillingAddress
    }

    struct OrderLine {
        id: OrderLineId,
        order_id: OrderId,
        product_code: ProductCode,
        order_quantitiy: OrderQuantity,
        price: Price
    }

    struct UnvalidatedOrder {
        order_id: String,
        customer_info: String,
        shipping_address: String
    }

    struct PlaceOrderEvent {

    }

    enum PlaceOrderError {
        ValidationErrors(Vec<ValidationError>),
        GeneralErrors(Vec<GeneralError>)
    }

    struct ValidationError {
        field_name: String,
        error_description: String
    }

    struct GeneralError {
        message: String,
    }

    // type​ PlaceOrder = UnvalidatedOrder -> Result<PlaceOrderEvents,PlaceOrderError>
    fn place_order(unvalidatedOrder: UnvalidatedOrder) -> Result<PlaceOrderEvent, PlaceOrderError> {
        return Ok(PlaceOrderEvent {})
    }

}

#[cfg(test)]
mod tests_modeling {
    use crate::ordertracking_domain::{WidgetCode, UnitQuantity};
    use uom::si::mass::kilogram;
    use uom::si::f32::*;


    #[test]
    fn valid_widgetcode() {
        let x = WidgetCode::create("W1234");
        
        assert!(x.is_ok());
    }
    #[test]
    fn valid_unitQuantity() {
        let x = UnitQuantity::create(1);
        
        assert!(x.is_ok());
    }

    #[test]
    fn valid_kilogramQuantity() {
        let x = Mass::new::<kilogram>(10.0);
        
    }
}

mod practice;

#[cfg(test)]
mod tests_practice;
