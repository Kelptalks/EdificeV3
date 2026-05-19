use crate::game_data::{
    player_data::{
        drones::{drone_actions::drone_actions::DroneActionError, drone_inventory::DroneInventory},
        game_entity::components::{action_outcome::ActionOutcome, entity_components::EntityComponent},
    },
    types::drone_item::DroneItem,
};

/// Holds a drone's item slots and crafting. No fuel — fuel is handled elsewhere.
#[derive(Clone)]
pub struct InventoryComponent {
    inventory: DroneInventory,
}

impl InventoryComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Inventory(self)
    }

    pub fn new() -> InventoryComponent {
        InventoryComponent {
            inventory: DroneInventory::new(),
        }
    }

    pub fn inventory(&self) -> &DroneInventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut DroneInventory {
        &mut self.inventory
    }

    /// Crafts one `item`, consuming its ingredients from this inventory.
    /// Busy cost is the item's craft time.
    pub fn craft(&mut self, item: DroneItem) -> ActionOutcome {
        if !item.is_craftable() {
            return ActionOutcome::failed(DroneActionError::UncraftableItem);
        }

        let craft_cost = item.get_craft_cost();

        // Verify every ingredient is present before consuming any.
        for slot in &craft_cost {
            let ingredient = slot.get_item().unwrap();
            if !self.inventory.has_item(ingredient, slot.get_quantity()) {
                return ActionOutcome::failed(DroneActionError::MissingItem);
            }
        }

        for slot in &craft_cost {
            let ingredient = slot.get_item().unwrap();
            self.inventory.remove_item(ingredient, slot.get_quantity());
        }

        self.inventory.add_item(item, 1);
        ActionOutcome::ok(item.get_craft_time())
    }
}
