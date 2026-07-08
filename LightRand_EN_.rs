use std::io;
use std::io::Write;

mod rand;

fn main() {
    println!("=== 🦟 MOSQUITO ENTROPY ===");

    let specializations = [
    "EVASIVE DASH ( Dashed into a wall )",
    "CLOAKING DEVICE ( Still got one-shotted )",
    "GRAPPLING HOOK ( Fell off the map )"
    ];

    let weapons = [
    "ARN-220 ( Wrong class weapon )",
    "DAGGER ( Missed the backstab )",
    "SWORD ( Basically a fourth dash )",
    "THROWING KNIVES ( Zero hits recorded )",
    "LH1 ( Spam click for headshots )",
    "RECURVE BOW ( Robin Hood cosplay )",
    "SH1900 ( Missed both, dead )",
    "M11 ( No damage past 2m )",
    "93R ( Zero damage burst )",
    "V9S ( RIP left click )",
    "M26 MATTER ( Shooting peas )",
    "XP-54 ( Meta slave weapon )",
    "SR-84 ( Useless on a crane )"
    ];

    let mut gadgets = vec! [
    "GOO GRENADE ( Blocked your own team )",
    "FRAG GRENADE ( Self damage machine )",
    "PYRO GRENADE ( Accidental teammate grill )",
    "SMOKE GRENADE ( Extinguishes fire but still useless )",
    "FLASHBANG ( Flashbanged yourself again )",
    "GLITCH GRENADE ( Direct counter to shields )",
    "TRACKING DART ( You cannot hide anymore )",
    "NULLIFIER ( No gadgets allowed here )",
    "VANISHING BOMB ( Voila, and I'm gone )",
    "H+ INFUSER ( Light turned into Medic )",
    "GRAVITY VORTEX ( Messing with map physics )",
    "GAS GRENADE ( Suffocating in your smoke )",
    "SONAR GRENADE ( Legal wallhack user )",
    "GATEWAY ( Now we are thinking with portals )",
    "BREACH CHARGE ( Making a brand new door )",
    "THERMAL BORE ( Long range wall puncher )"
    ];

    let wait_for_click = |text: &str| {
        println!("{}", text);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    };

    println!("System is ready for build generation");

    // 1. spec gen
    wait_for_click("👉 Press [Enter] to roll your ABILITY...");
    let spec_idx = rand::gen_range(0, (specializations.len() - 1) as u32) as usize;
    let chosen_spec = specializations[spec_idx];
    println!(" 🛡️ABILITY: {}", chosen_spec);

    // 2. gun gen
    wait_for_click("👉 Press [Enter] to roll your WEAPON...");
    let weapon_idx = rand::gen_range(0, (weapons.len() - 1) as u32) as usize;
    let chosen_weapon = weapons[weapon_idx];
    println!(" 🔫WEAPON: {}", chosen_weapon);

    // 3. gadget gen
    wait_for_click("👉 Press [Enter] to roll your GADGETS...");

    let g1_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot1 = gadgets.remove(g1_idx);

    let g2_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot2 = gadgets.remove(g2_idx);

    let g3_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot3 = gadgets.remove(g3_idx);

    println!(" 🎒 GADGET 1:  {}", gadget_slot1);
    println!(" 🎒 GADGET 2:  {}", gadget_slot2);
    println!(" 🎒 GADGET 3:  {}", gadget_slot3);

    println!("=== ENTROPY HAS SPOKEN! GO WIN THIS! ===");

    // Program end
    println!("Press [Enter] to close the program...");
    io::stdout().flush().unwrap();
    let mut exit_buf = String::new();
    io::stdin().read_line(&mut exit_buf).unwrap();
}
