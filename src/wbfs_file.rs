// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuel.quarneti@proton.me>
// SPDX-License-Identifier: GPL-2.0-only

use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{bail, Result};

extern "C" {
    fn conv_to_wbfs(
        filename: *const ::std::os::raw::c_char,
        dest_dir: *const ::std::os::raw::c_char,
    );
}

pub fn conv_to_wbfs_wrapper(src: &str, dest: &str) {
    let src = CString::new(src).unwrap();
    let dest = CString::new(dest).unwrap();

    unsafe {
        conv_to_wbfs(src.as_ptr(), dest.as_ptr());
    };
}

fn get_wbfs_id_and_title(path: &Path) -> Result<(String, String)> {
    let mut file = File::open(&path)?;

    // check if the file is a wbfs file
    let mut magic = [0u8; 0x4];
    file.read_exact(&mut magic)?;
    if magic != [0x57, 0x42, 0x46, 0x53] {
        bail!("Invalid wbfs file");
    }

    // read the id
    file.seek(SeekFrom::Start(0x200))?;
    let mut id = [0u8; 0x6];
    file.read_exact(&mut id)?;
    let id = String::from_utf8(id.to_vec())?;

    // read the title
    file.seek(SeekFrom::Start(0x220))?;
    let mut title = [0u8; 0x40];
    file.read_exact(&mut title)?;
    let title = String::from_utf8(title.to_vec())?;
    let title = title.trim_matches(char::from(0)).to_string();

    Ok((id, title))
}

pub fn copy_wbfs_file(src: &Path, dest: &Path) -> Result<()> {
    let (id, title) = get_wbfs_id_and_title(src)?;
    let dest_dir = dest.join(format!("{} [{}]", title, id));

    if !dest_dir.exists() {
        std::fs::create_dir(&dest_dir)?;
    }

    let dest_file = dest_dir.join(src.file_name().unwrap());
    std::fs::copy(src, dest_file)?;

    // copy eventual wbf[1-3] files
    for i in 1..=3 {
        let src = src.with_extension(format!("wbf{}", i));
        if src.exists() {
            let dest_file = dest_dir.join(src.file_name().unwrap());
            std::fs::copy(src, dest_file)?;
        }
    }

    Ok(())
}
