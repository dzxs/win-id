extern crate winreg;
use std::io;
use winreg::enums::*;
use winreg::RegKey;
use uuid::Uuid;

fn main() -> io::Result<()> {
    let uuid = Uuid::new_v4();
    let new_machine_id = format!("{{{}}}", uuid.to_hyphenated().encode_upper(&mut Uuid::encode_buffer()));
    let hklm  = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (sqm_client, _disp) = hklm.create_subkey("SOFTWARE\\Microsoft\\SQMClient")?;
    let machine_id: String = sqm_client.get_value("MachineId")?;
    println!("旧的 MachineId = {}", machine_id);
    sqm_client.set_value("MachineId", &new_machine_id)?;
    let machine_id_modified: String = sqm_client.get_value("MachineId")?;
    println!("新的 MachineId = {}", machine_id_modified);

    Ok(())

}
