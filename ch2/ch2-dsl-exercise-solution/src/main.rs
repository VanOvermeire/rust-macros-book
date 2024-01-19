use std::ops::{Add, Sub};

#[derive(Debug)]
struct Account {
    money: u32,
}

impl Account {
    fn add(&mut self, money: u32) {
        self.money = self.money.add(money)
    }

    fn subtract(&mut self, money: u32) {
        self.money = self.money.sub(money)
    }
}

enum Currency {
    Euro,
    Dollar
}

impl From<&str> for Currency {
    fn from(value: &str) -> Self {
        if value.contains("euro") {
            Currency::Euro
        } else {
            // simple fallback to dollars
            Currency::Dollar
        }
    }
}

impl Currency {
    fn calculate(&self, amount: u32) -> u32 {
        match self {
            Currency::Euro => amount,
            Currency::Dollar => amount * 2
        }
    }
}

macro_rules! exchange {
    (Give $amount:literal $currency:literal to $name:ident) => {
        let curr: Currency = $currency.into();
        $name.add(curr.calculate($amount))
    }
    // others are similar to the above
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Account;
    use crate::Currency;

    #[test]
    fn should_handle_currencies_for_giving() {
        let mut the_poor = Account {
            money: 0,
        };

        exchange!(Give 10 "euros" to the_poor);
        exchange!(Give 10 "dollars" to the_poor);
        exchange!(Give 1 "euro" to the_poor);

        assert_eq!(the_poor.money, 31);
    }
}