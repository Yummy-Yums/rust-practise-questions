pub mod cell_program {
    use std::cell::{Cell, RefCell};

    #[derive(Debug, PartialEq)]
    pub struct PhoneModel {
        pub company_name: String,
        pub model_name: String,
        pub screen_size: f32,
        pub memory: usize,
        pub date_issued: u32,
        pub on_sale: Cell<bool>,
    }

    #[derive(Debug, PartialEq)]
    pub(crate) struct BankDetails {
        pub balance: f64,
        pub location: String,
        pub customer_count: RefCell<usize>,
    }
}

mod test {
    use std::cell::{Cell, RefCell};

    #[test]
    pub fn test_cell_program() {
        let super_phone_3000 = crate::chp3::cell_prgoram::cell_program::PhoneModel {
            company_name: "YY Electronics".to_string(),
            model_name: "Super Phone 3000".to_string(),
            screen_size: 7.5,
            memory: 4_000_000,
            date_issued: 2020,
            on_sale: Cell::new(true),
        };

        super_phone_3000.on_sale.set(false);
        // super_phone_3000.screen_size = 2.3;

        println!("{:?}", super_phone_3000.on_sale);

        let customer = crate::chp3::cell_prgoram::cell_program::BankDetails {
            balance: 0.0,
            location: String::from("tadi"),
            customer_count: RefCell::new(23)
        };

        if customer.balance == 0.0 {
            customer
                .customer_count
                .replace(22);
        }

        assert_eq!(super_phone_3000.on_sale.get(), false);
        assert_eq!(customer.customer_count.take(), 22);
    }

}