
#[derive(Clone)]
pub struct TraitPowered {
    pub stored_power: u32,
    pub power_consumed: u32,

}

impl TraitPowered {
    pub fn new() -> TraitPowered {
        TraitPowered {
            stored_power: 0,
            power_consumed: 0,
        }
    }

    pub fn tik(&mut self, power_needed: u32) -> bool {
        if self.stored_power > power_needed {
            self.stored_power -= power_needed;
            true
        }
        else {
            false
        }
    }

    pub fn consume_power(&mut self, amount: u32) {
        if self.stored_power > amount {
            self.stored_power -= amount;
        }
        else {
            self.stored_power = 0;
        }
    }
}