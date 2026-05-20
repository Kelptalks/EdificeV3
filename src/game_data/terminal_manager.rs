use std::io::{stdout, Write};
use std::sync::mpsc::{self, Receiver};

use crossterm::{
    cursor::MoveTo,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::game_data::types::BlockTexture;

// Top-left corner of the 2×2 button in terminal columns/rows
const BTN_COL: u16 = 0;
const BTN_ROW: u16 = 0;

pub enum TerminalEvent {
    Line(String),
    ButtonClicked,
}

pub struct TerminalManager {
    rx: Receiver<TerminalEvent>,
}

impl TerminalManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            enable_raw_mode().ok();
            execute!(stdout(), EnableMouseCapture, Clear(ClearType::All), MoveTo(0, 0)).ok();

            draw_button(false);

            loop {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        if key.code == KeyCode::Esc {
                            break;
                        }
                        // Accumulate printable chars into a line on Enter
                        if let KeyCode::Char(c) = key.code {
                            // single-char passthrough for now; extend if you want a full input buffer
                            let _ = tx.send(TerminalEvent::Line(c.to_string()));
                        }
                    }
                    Ok(Event::Mouse(m)) => {
                        let on_btn = m.column >= BTN_COL
                            && m.column < BTN_COL + 2
                            && m.row >= BTN_ROW
                            && m.row < BTN_ROW + 2;

                        match m.kind {
                            MouseEventKind::Down(MouseButton::Left) if on_btn => {
                                draw_button(true);
                                let _ = tx.send(TerminalEvent::ButtonClicked);
                            }
                            MouseEventKind::Up(MouseButton::Left) => {
                                draw_button(false);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => break,
                    _ => {}
                }
            }

            execute!(stdout(), DisableMouseCapture, ResetColor).ok();
            disable_raw_mode().ok();
        });

        Self { rx }
    }

    pub fn drain(&self) -> Vec<TerminalEvent> {
        self.rx.try_iter().collect()
    }
}

fn draw_button(pressed: bool) {
    let bg = if pressed { Color::White } else { Color::DarkGrey };
    let fg = if pressed { Color::Black } else { Color::White };
    let mut out = stdout();
    // Row 0
    execute!(
        out,
        MoveTo(BTN_COL, BTN_ROW),
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        Print("[]"),
        // Row 1
        MoveTo(BTN_COL, BTN_ROW + 1),
        Print("[]"),
        ResetColor,
    ).ok();
    out.flush().ok();
}

pub fn parse_block_texture(s: &str) -> Option<BlockTexture> {
    if let Ok(id) = s.parse::<u16>() {
        return block_texture_from_id(id);
    }
    let lower = s.to_lowercase();
    let result = match lower.as_str() {
        "air"               => BlockTexture::Air,
        "stone"             => BlockTexture::Stone,
        "grass"             => BlockTexture::Grass,
        "dirt"              => BlockTexture::Dirt,
        "browntrunk"        => BlockTexture::BrownTrunk,
        "leaves"            => BlockTexture::Leaves,
        "purpletrunk"       => BlockTexture::PurpleTrunk,
        "iron"              => BlockTexture::Iron,
        "granite"           => BlockTexture::Granite,
        "sand"              => BlockTexture::Sand,
        "copperore"         => BlockTexture::CopperOre,
        "pinkfungus"        => BlockTexture::PinkFungus,
        "bluegrass"         => BlockTexture::BlueGrass,
        "mushroomstem"      => BlockTexture::MushroomStem,
        "pinkmushroomblock" => BlockTexture::PinkMushroomBlock,
        "mudbricks"         => BlockTexture::MudBricks,
        "orangefungus"      => BlockTexture::OrangeFungus,
        "stonebrick"        => BlockTexture::StoneBrick,
        "flowerstonebrick"  => BlockTexture::FlowerStoneBrick,
        "scaffolding"       => BlockTexture::Scaffolding,
        "pinkcloud"         => BlockTexture::PinkCloud,
        "dandistem"         => BlockTexture::DandiStem,
        "hive"              => BlockTexture::Hive,
        "cobblestone"       => BlockTexture::CobbleStone,
        "magma"             => BlockTexture::Magma,
        "core"              => BlockTexture::Core,
        "lbm"               => BlockTexture::LBM,
        "crackedearth"      => BlockTexture::CrackedEarth,
        "debug"             => BlockTexture::Debug,
        "water"             => BlockTexture::Water,
        "glass"             => BlockTexture::Glass,
        "redbrick"          => BlockTexture::RedBrick,
        "dronecontroler"    => BlockTexture::DroneControler,
        "ironore"           => BlockTexture::IronOre,
        "bluemushroom"      => BlockTexture::BlueMushroom,
        "smokestack"        => BlockTexture::SmokeStack,
        "brownplanks"       => BlockTexture::BrownPlanks,
        "cloudblock"        => BlockTexture::CloudBlock,
        "purpleplanks"      => BlockTexture::PurplePlanks,
        "furnaceoff"        => BlockTexture::FurnaceOff,
        "furnaceon"         => BlockTexture::FurnaceOn,
        "titaniumore"       => BlockTexture::TitaniumOre,
        _ => return None,
    };
    Some(result)
}

fn block_texture_from_id(id: u16) -> Option<BlockTexture> {
    if id > 399 {
        return None;
    }
    Some(unsafe { std::mem::transmute::<u16, BlockTexture>(id) })
}
