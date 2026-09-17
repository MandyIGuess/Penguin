use eframe::egui;

use crate::savefile::{constants::*, saveslot::SaveSlot};

pub struct SlotView {
    world_edit_index: usize,
    stage_edit_index: usize,
    player_edit_index: usize,
}

/*fn get_house_type_string(house_type: StartingMushroomKind) -> String {
    match house_type {
        StartingMushroomKind::None => "None",
        StartingMushroomKind::Star => "Star",
        StartingMushroomKind::Item => "Item",
        StartingMushroomKind::OneUp => "1-Up",
        StartingMushroomKind::StarRescue => "Star (Rescue)",
        StartingMushroomKind::ItemRescue => "Item (Rescue)",
        StartingMushroomKind::OneUpRescue => "1-Up (Rescue)",
    }
    .to_string()
}*/

fn get_stage_name_string(stage_index: usize) -> String {
    match stage_index {
        0..=9 => format!("Stage {}", stage_index + 1),
        14 | 15 => format!("Tower {}", stage_index - 13),
        23 => format!("Final Castle"),
        24 => format!("Castle"),
        30 => format!("Switch Palace"),
        32 => format!("Challenge House"),
        33 => format!("Music House"),
        // Skipping idx 40, which is used for some cutscenes and the credits
        42 => format!("Invalid Stage ID"),
        _ => format!("(unused slot {})", stage_index + 1),
    }
}

impl SlotView {
    pub fn new() -> Self {
        Self {
            world_edit_index: 0,
            stage_edit_index: 0,
            player_edit_index: 0,
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui, slot: &mut SaveSlot) {
        // game completion, world state
        ui.add_space(3.0);
        ui.horizontal_centered(|ui|{
            egui::ScrollArea::horizontal().show(ui, |ui|{
            // game completion
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.label("Completion flags");
                    // game completion flags
                    let labels = [
                        "Save empty",
                        "Final boss beaten",
                        "All goals",
                        "All star coins (W1-W8)",
                        "All star coins (W9)",
                        "Game completed",
                        "Super Guide triggered"  
                    ];
        
                    for (i, label) in labels.iter().enumerate() {
                        let mut is_checked = (slot.game_completion_flags & (1 << i)) != 0;
        
                        if ui.checkbox(&mut is_checked, *label).changed() {
                            if is_checked {
                                slot.game_completion_flags |= 1 << i;
                            } else {
                                slot.game_completion_flags &= !(1 << i);
                            }
                        }
                    }
                });
    
                ui.vertical(|ui| {
                    ui.label("Enabled switches");
                    // switch palaces
                    let labels = [
                        "Red",
                        "Green",
                        "Yellow",
                        "Blue"
                    ];
        
                    for (i, label) in labels.iter().enumerate() {
                        let mut is_checked = (slot.switch_palace_flags & (1 << i)) != 0;
        
                        if ui.checkbox(&mut is_checked, *label).changed() {
                            if is_checked {
                                slot.switch_palace_flags |= 1 << i;
                            } else {
                                slot.switch_palace_flags &= !(1 << i);
                            }
                        }
                    }
                });
                ui.vertical(|ui|{
                    ui.label("Hint movie bought");
                    egui::ScrollArea::vertical().show(ui, |ui|{
                        for (i, title) in HINT_MOVIE_TITLES.iter().enumerate().take(ACTUAL_HINT_MOVIE_COUNT) {
                            ui.checkbox(&mut slot.hint_movie_bought[i], *title);
                        }
                    });
                });
            });

            ui.separator();

            // game state
            ui.vertical(|ui|{
                ui.horizontal(|ui|{
                    ui.vertical(|ui|{
                        ui.label("Score");
                        ui.add(
                            egui::DragValue::new(&mut slot.ingame_score)
                            .speed(1)
                            .range(0..=MAX_SCORE)
                        );
                    });
                    ui.add_space(3.0);
                    ui.vertical(|ui|{
                        ui.label("Spent star coins");
                        ui.add(
                            egui::DragValue::new(&mut slot.star_coins_spent)
                            .speed(1)
                            .range(0..=u16::MAX)
                        );
                    });
                });
                ui.add_space(3.0);

                ui.label("Item stock");
                for (i, item_name) in ITEM_MENU_POWERUP_NAMES.iter().enumerate() {
                    ui.horizontal(|ui|{
                        ui.add(
                            egui::DragValue::new(&mut slot.item_stock[i])
                            .speed(1)
                            .range(0..=POWERUP_STOCK_MAX)          
                        );
                        ui.label(*item_name);
                    });

                }                
            });
            
            ui.separator();


            // world state
            ui.vertical(|ui|{
                // w5 vine reshuffle
                ui.label("W5 vine reshuffle counter");
                ui.add(
                    egui::DragValue::new(&mut slot.w5_vine_reshuffle_counter)
                    .speed(1)
                    .range(0..=255)
                );
                
                ui.add_space(3.0);

                egui::ComboBox::from_label("Selected World")
                .selected_text(
                    format!("World {}", self.world_edit_index + 1)
                )
                .show_ui(ui, |ui|{
                    for i in 0..ACTUAL_WORLD_COUNT {
                        ui.selectable_value(
                            &mut self.world_edit_index,
                            i,
                            format!("World {}", i + 1)
                        );
                    }
                });
                
                ui.vertical(|ui|{
                    // stage completion
                    egui::ComboBox::from_label("Selected Stage")
                    .selected_text(
                        get_stage_name_string(self.stage_edit_index)
                    )
                    .show_ui(ui, |ui|{
                        for i in 0..=STAGE_COUNT {
                            ui.selectable_value(
                                &mut self.stage_edit_index,
                                i,
                                get_stage_name_string(i)
                            );
                        }
                    });
                    
                    ui.label("Stage Completion Flags");
                    let labels = [
                        "Star Coin 1",
                        "Star Coin 2",
                        "Star Coin 3",
                        "Goal (Normal)",
                        "Goal (Secret)",
                        "Super Guide (Normal)",
                        "Super Guide (Secret)",
                        "Visible in Star Coins Menu",
                    ];
                    let flags = [
                        1, 2, 4, 0x10, 0x20, 0x80, 0x100, 0x200
                    ];
                    for i in 0..labels.len() {
                        let mut is_checked = (
                            slot.stage_completion_flags[self.world_edit_index][self.stage_edit_index]
                                & flags[i]
                        ) != 0;
                        
                        if ui.checkbox(&mut is_checked, labels[i]).changed() {
                            if is_checked {
                                slot.stage_completion_flags[self.world_edit_index][self.stage_edit_index]
                                    |= flags[i]
                            } else {
                                slot.stage_completion_flags[self.world_edit_index][self.stage_edit_index]
                                    &= !flags[i];
                            }
                        }
                    }
                });
            });

            ui.separator();

            // player information
            ui.vertical(|ui|{
                
                egui::ComboBox::from_label("Current world")
                    .selected_text(format!("World {}", slot.cur_world + 1))
                    .show_ui(ui, |ui|{
                        for i in 0..ACTUAL_WORLD_COUNT as u8 {
                            ui.selectable_value(
                                &mut slot.cur_world,
                                i,
                            format!("World {}", i + 1)
                        );
                    }
                });
                ui.label("Current subworld")
                .on_hover_text("An example of a 'subworld' is the second half of World 3.");
                ui.add(
                    egui::DragValue::new(&mut slot.cur_subworld)
                    .speed(1)
                    .range(0..=1)
                );
                ui.label("Current path node");
                ui.add(
                    egui::DragValue::new(&mut slot.cur_path_node)
                    .speed(1)
                    .range(0..=u8::MAX)
                );
                // *player* information

                ui.vertical(|ui|{
                    egui::ComboBox::from_label("Selected player")
                    .selected_text(format!("Player {}", self.player_edit_index + 1))
                    .show_ui(ui, |ui|{
                        for i in 0..PLAYER_COUNT {
                            ui.selectable_value(
                                &mut self.player_edit_index,
                                i,
                                format!("Player {}", i + 1)
                            );
                        }
                    });
                    egui::ComboBox::from_label("Character")
                    .selected_text(
                        match slot.player_character[self.player_edit_index] {
                            PlayerCharacter::Mario => "Mario",
                            PlayerCharacter::Luigi => "Luigi",
                            PlayerCharacter::BlueToad => "Blue Toad",
                            PlayerCharacter::YellowToad => "Yellow Toad"
                        }
                    )
                    .show_ui(ui, |ui|{
                        for (i, name) in PLAYER_NAMES.iter().enumerate() {
                            ui.selectable_value(
                                &mut slot.player_character[self.player_edit_index],
                                match i {
                                    0 => PlayerCharacter::Mario,
                                    1 => PlayerCharacter::Luigi,
                                    2 => PlayerCharacter::BlueToad,
                                    3 => PlayerCharacter::YellowToad,
                                    _ => PlayerCharacter::Mario
                                },
                                *name
                            ).on_hover_text("Player 1 will always be Mario.");
                        }
                    });
                    ui.horizontal(|ui|{
                        ui.vertical(|ui|{
                            ui.label("Continues");
                            ui.add(
                                egui::DragValue::new(&mut slot.player_continues[self.player_edit_index])
                                .speed(1)
                                .range(0..=u8::MAX)
                            );
                        });
                        ui.vertical(|ui|{
                            ui.label("Coins")
                            .on_hover_text("In multiplayer, these numbers add up to the coin count seen on-screen.");
                        ui.add(
                                egui::DragValue::new(&mut slot.player_coins[self.player_edit_index])
                                .speed(1)
                                .range(0..=u8::MAX)
                            );
                        });
                        ui.vertical(|ui|{
                            ui.label("Lives");
                            ui.add(
                                egui::DragValue::new(&mut slot.player_lives[self.player_edit_index])
                                .speed(1)
                                .range(0..=PLAYER_LIFE_MAX)
                            );
                        });
                    });
                    ui.add_space(3.0);
                    egui::ComboBox::from_label("Powerup")
                    .selected_text(
                        match slot.player_powerup[self.player_edit_index] {
                            PlayerPowerup::None => "None",
                            PlayerPowerup::Mushroom => "Mushroom",
                            PlayerPowerup::FireFlower => "Fire Flower",
                            PlayerPowerup::MiniMushroom => "Mini Mushroom",
                            PlayerPowerup::PropellerMushroom => "Propeller Mushroom",
                            PlayerPowerup::PenguinSuit => "Penguin Suit",
                            PlayerPowerup::IceFlower => "Ice Flower",
                            PlayerPowerup::HammerSuit => "Hammer Suit",
                        }
                    ).show_ui(ui, |ui|{
                        for (i, status) in PLAYER_POWERUP_STATUS.iter().enumerate() {
                            let val = match i {
                                0 => PlayerPowerup::None,
                                1 => PlayerPowerup::Mushroom,
                                2 => PlayerPowerup::FireFlower,
                                3 => PlayerPowerup::MiniMushroom,
                                4 => PlayerPowerup::PropellerMushroom,
                                5 => PlayerPowerup::PenguinSuit,
                                6 => PlayerPowerup::IceFlower,
                                7 => PlayerPowerup::HammerSuit,
                                _ => PlayerPowerup::None                                    
                            };
                            ui.selectable_value(
                                &mut slot.player_powerup[self.player_edit_index],
                                val,
                                *status
                            );

                        }
                    });
                    
                    // omitting the other flags for now since they don't do anything
                    {
                        let mut is_checked = (slot.player_spawn_flags[self.player_edit_index] & PlayerCreationFlags::StarPower.bits()) != 0;
                        if ui.checkbox(&mut is_checked, "Star Power").changed() {
                            if is_checked {
                                slot.player_spawn_flags[self.player_edit_index] |= PlayerCreationFlags::StarPower.bits();
                            } else {
                                slot.player_spawn_flags[self.player_edit_index] &= !PlayerCreationFlags::StarPower.bits();
                            }
                        }
                    }
                });
            });
        });
        });
    }
}
