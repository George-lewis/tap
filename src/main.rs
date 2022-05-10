use std::{
    borrow::Cow,
    io::{stdin, stdout, Read, Write},
};

const BUF_SIZE: usize = libc::BUFSIZ as usize;

fn main() -> Result<(), Cow<'static, str>> {
    let arg = std::env::args().nth(1);
    let fname = arg.as_deref().ok_or("missing filename argument")?;

    let mut file =
        std::fs::File::create(fname).map_err(|e| format!("couldn't open file: {fname} {e}"))?;

    let stdin = stdin();
    let mut stdin = stdin.lock();

    let stdout = stdout();
    let mut stdout = stdout.lock();

    let mut buf = [0_u8; BUF_SIZE];

    while let Ok(n) = stdin.read(&mut buf[..]) {
        if n == 0 {
            break;
        }

        stdout
            .write_all(&buf[..n])
            .map_err(|e| format!("couldn't write to stdout: {e}"))?;
        file.write_all(&buf[..n])
            .map_err(|e| format!("couldn't write to file: {fname} {e}"))?;
    }

    Ok(())
}
