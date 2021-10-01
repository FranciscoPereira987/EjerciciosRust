mod lector;
mod lcs_por_linea;
mod diff;

fn main() -> Result<(), String>{
    let diff = diff::Diff::new("unArchivo.txt", "otroArchivo.txt")?;

    diff.diff();

    Ok(())
}


