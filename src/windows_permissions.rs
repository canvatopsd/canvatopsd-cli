use ::windows_permissions::{
    LocalBox, SecurityDescriptor,
    constants::{SeObjectType::SE_FILE_OBJECT, SecurityInformation},
    utilities::current_process_sid,
    wrappers::SetSecurityInfo,
};
use anyhow::{Context, Result};
use std::{
    fs::{File, OpenOptions},
    os::windows::{
        fs::OpenOptionsExt,
        io::{AsRawHandle, FromRawHandle},
    },
    path::Path,
};
use windows_sys::Win32::{
    Foundation::INVALID_HANDLE_VALUE,
    Storage::FileSystem::{
        FILE_FLAG_BACKUP_SEMANTICS, FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, READ_CONTROL, ReOpenFile, WRITE_DAC,
    },
};

fn restrict(file: &mut File) -> Result<()> {
    let sid = current_process_sid().context("Cannot identify the Windows credential owner")?;
    let inheritance = if file.metadata()?.is_dir() {
        "OICI"
    } else {
        ""
    };
    let descriptor: LocalBox<SecurityDescriptor> =
        format!("D:P(A;{inheritance};FA;;;{sid})").parse()?;
    SetSecurityInfo(
        file,
        SE_FILE_OBJECT,
        SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
        None,
        None,
        Some(
            descriptor
                .dacl()
                .context("Missing credential access rules")?,
        ),
        None,
    )
    .context("Cannot restrict Windows credential access to the current user")
}

pub fn protect_path(path: &Path) -> Result<()> {
    let mut file = OpenOptions::new()
        .access_mode(READ_CONTROL | WRITE_DAC | FILE_READ_ATTRIBUTES)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .context("Cannot open credential path to restrict Windows access")?;
    restrict(&mut file)
}

pub fn protect_file(file: &File) -> Result<()> {
    // Reopen the same object with WRITE_DAC; ordinary write handles lack this right.
    let handle = unsafe {
        ReOpenFile(
            file.as_raw_handle(),
            READ_CONTROL | WRITE_DAC | FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            FILE_FLAG_BACKUP_SEMANTICS,
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error())
            .context("Cannot reopen credential file to restrict Windows access");
    }
    // ReOpenFile returned a new owned handle; File closes it after applying the ACL.
    let mut reopened = unsafe { File::from_raw_handle(handle) };
    restrict(&mut reopened)
}
