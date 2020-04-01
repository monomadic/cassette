pub(crate) fn read_file(pathbuf: std::path::PathBuf) -> crate::CassetteResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf.clone()).map_err(|_| {
        failure::format_err!(
            "Could not open or read file: {}",
            pathbuf.to_str().unwrap_or("")
        )
    })?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
