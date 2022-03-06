fn main() {}

mod ordertracking_domain {
    struct WidgetCode(String);
    struct GizmoCode(String);
    enum ProductCode{
        Widget(WidgetCode),
        Gizmo(GizmoCode)
    }

    struct KilogramQuantity(f32);

    struct UnitQuantity(i32);

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

}

mod practice;

#[cfg(test)]
mod tests_practice;
