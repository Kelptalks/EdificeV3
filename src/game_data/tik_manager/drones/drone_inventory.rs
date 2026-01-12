use std::ops::Index;

use crate::game_data::types::drone_item::DroneItem;


pub struct InventorySlot {
    item_quantity: i32,
    item_type: Option<DroneItem>
}

impl InventorySlot {
    pub fn new() -> InventorySlot {
        InventorySlot {
            item_quantity: 0,
            item_type: None,
        }
    }

    pub fn get_item(&self) -> Option<DroneItem> {
        return self.item_type;
    }
    pub fn set_item(&mut self, drone_item: DroneItem) {
        self.item_type = Some(drone_item);
        self.item_quantity = 0;
    }

    pub fn get_quantity(&self) -> i32 {
        return self.item_quantity;
    }
    pub fn set_quantity(&mut self, quantity: i32) {
        self.item_quantity = quantity;
    }
    pub fn mod_quantity(&mut self, quantity: i32) {
        self.item_quantity += quantity;
        if self.item_quantity <= 0 {
            self.item_type = None;
        }
    }


}

pub struct DroneInventory {
    slots: Vec<InventorySlot>,
    total_slots: u32,
    total_items: u32,
}

impl DroneInventory {
    pub fn new() -> DroneInventory {
        let mut drone_inventory = DroneInventory {
            slots: Vec::new(),
            total_slots: 9,
            total_items: 0,
        };

        // init inventory slots
        for i in 0..drone_inventory.total_slots {
            drone_inventory.slots.push(InventorySlot::new());
        }

        return drone_inventory;
    }

    // Get a slot at an index
    pub fn get_slot_at_index(&self, index: u32) -> Option<&InventorySlot> {
        if index > self.total_slots {
            return None;
        }
        return Some(&self.slots[index as usize]);
    }

    // Has the an amount of item
    pub fn has_item_amount(&self, drone_item: DroneItem) -> i32 {
        for slot in &self.slots {
            if let Some(item_type) = slot.get_item() {
                if item_type == drone_item {
                    // If there is enough of the item
                    return slot.get_quantity();
                }
            }
        }
        return 0;
    }

    // Checks if there is enough of an item
    pub fn has_item(&self, drone_item: DroneItem, quantity: i32) -> bool {
        for slot in &self.slots {
            if let Some(item_type) = slot.get_item() {
                if item_type == drone_item {
                    // If there is enough of the item
                    if quantity <= slot.get_quantity() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    // Add an item to the inventory
    pub fn add_item(&mut self, drone_item: DroneItem, quantity: i32) {
        // Try to add to existing slot and get first free slot
        let mut first_free_slot = None;
        for slot in &mut self.slots {
            if let Some(item_type) = slot.item_type {
                if item_type == drone_item {
                    slot.mod_quantity(quantity);
                    return;
                }
            }
            else {
                first_free_slot = Some(slot);
            }
        }

        // If could not add to existing stack try and add to first empty
        if let Some(slot) = first_free_slot {
            slot.set_item(drone_item);
            slot.set_quantity(quantity);
            return;
        }
        else {
            println!("Cannot add Inventory full");
            return;
        }
    }


    // remove item from inventory | Return true if removal succsessfull 
    pub fn remove_item(&mut self, drone_item: DroneItem, quantity: i32) -> bool {
        // Try to add to existing slot and get first free slot
        for slot in &mut self.slots {
            if let Some(item_type) = slot.get_item() {
                if item_type == drone_item {
                    // If there is enough of the item to remove
                    if quantity <= slot.get_quantity() {
                        slot.mod_quantity(-quantity);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    // Get the item type in a slot
    pub fn get_slot_item_type(&self, slot_index: usize) -> Option<DroneItem> {
        if slot_index < self.total_slots as usize {
            return self.slots[slot_index].get_item();
        }
        return None;
    }

    // get the amount of items in a slot
    pub fn get_slot_item_quantity(&self, slot_index: usize) -> i32 {
        if slot_index < self.total_slots as usize {
            return self.slots[slot_index].get_quantity();
        }
        return 0;
    }
}