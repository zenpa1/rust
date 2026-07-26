use game::LootCache;

mod game {
    #[derive(Debug)]
    pub struct LootCache<T, U> {
        primary: T,
        secondary: U,
    }

    impl<T, U> LootCache<T, U> {
        // impl knows that T and U are generic types from declaration
        pub fn primary(&self) -> &T {
            // Borrow/read item in Primary slot
            &self.primary
        }

        pub fn transmute<V, W>(self, other: LootCache<V, W>) -> LootCache<T, W> {
            // Consume both LootCaches to make a new one
            LootCache {
                primary: self.primary,
                secondary: other.secondary,
            }
        }

        pub fn new(primary: T, secondary: U) -> Self {
            Self { primary, secondary }
        }
    }

    impl LootCache<i32, i32> {
        // impl for LootCache with two i32 values
        pub fn calculate_wealth(&self) -> i32 {
            &self.primary + &self.secondary
        }
    }
}

fn main() {
    let lc1 = LootCache::new(5, 10);
    let lc2 = LootCache::new("hello", 'x');

    println!("lc1 primary: {}", lc1.primary());
    println!("lc1 sum: {}", lc1.calculate_wealth());

    println!("lc1 contents: {lc1:?}");
    println!("lc2 contents: {lc2:?}");
    println!("lc3 contents: {:?}", lc1.transmute(lc2));
}
