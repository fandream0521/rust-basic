mod front_of_house {
    pub use serving::take_order;
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn _seat_at_table() {}
    }

    fn serve_order() {}

    mod serving {
        pub fn take_order() {
            super::serve_order();
        }
        fn _serve_order() {}
        fn _take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    front_of_house::take_order();
}
