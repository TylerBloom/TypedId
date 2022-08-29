#[cfg(test)]
mod tests {
    use typed_id::id_type;

    id_type!(u32, Customer);
    id_type!(u32, Order);

    struct Customer {
        id: CustomerId,
        orders: Vec<OrderId>,
    }

    struct Order {
        id: OrderId,
    }

    impl Customer {
        fn has_order(&self, o_id: OrderId) -> bool {
            self.orders.iter().find(|&o| *o == o_id).is_some()
        }
    }

    #[test]
    fn basic_convertion() {
        let id = 42;

        let mut customer = Customer {
            id: id.into(),
            orders: Vec::new(),
        };
        let order = Order { id: id.into() };

        customer.orders.push(order.id);

        assert_eq!(id, *customer.id);
        assert_eq!(id, *order.id);

        assert!(customer.has_order(order.id));
        assert!(customer.has_order(id.into()));
        assert!(customer.has_order(customer.id.convert()));
    }

    #[test]
    fn basic_strings() {
        let id = 42;
        let t_id: CustomerId = id.into();
        assert_eq!(id.to_string(), t_id.to_string());
        assert_eq!(format!("{t_id:?}"), String::from("TypedId(42)"));
    }
}
