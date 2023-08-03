macro_rules! coordinate_response_generate {
    ($resp:ty,$val:ty,$ret:tt) => {
        impl CoordinateResponse for $resp {
            type Value = $val;
            fn code(&self) -> i32 {
                self.code
            }
            fn message(&self) -> &str {
                self.message.as_str()
            }
            fn value(self) -> Self::Value {
                self.$ret
            }
        }
    };
}
