use clearscreen;

pub fn starting_up() {
    clearscreen::clear().expect("failed to clear screen");
    let msg = format!(
        r"
    ______      __           __      __            
    / ____/___ _/ /______  __/ /___ _/ /_____  _____
   / /   / __ `/ / ___/ / / / / __ `/ __/ __ \/ ___/
  / /___/ /_/ / / /__/ /_/ / / /_/ / /_/ /_/ / /    
  \____/\__,_/_/\___/\__,_/_/\__,_/\__/\____/_/     
                                                   
    A calculator which follows BODMAS rule.
    "
    );
    println!("{msg}\n");
}
