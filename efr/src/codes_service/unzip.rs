use std::io::{Cursor, Read};
use zip::{ZipArchive, result::ZipError};

use crate::codes_service::{CodesHeader, EfrCodesError};

impl CodesHeader {
    pub fn unzip_xml(bytes: &[u8]) -> Result<String, EfrCodesError> {
        let reader = Cursor::new(bytes);
        let mut archive = ZipArchive::new(reader)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            if file.is_file() {
                let mut contents = String::with_capacity(file.size() as usize);
                if let Err(err) = file.read_to_string(&mut contents) {
                    return Err(EfrCodesError::Zip(ZipError::Io(err)));
                }
                return Ok(contents);
            }
        }

        Err(EfrCodesError::NoXmlCodesFileInsideZip)
    }
}
