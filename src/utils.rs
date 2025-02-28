use crate::prelude::*;

pub fn export_data_to_file(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    file.write_all(data)?;

    Ok(())
}
