use::std::{
    io::{
        BufRead,
        BufReader
    },

    fs::File
};

pub fn read_file_lines(path: &str) -> Result<Vec<String>, String>{

    if let Ok(un_archivo) = File::open(path){
        Ok(convertir_lineas_a_vector(un_archivo))
    }else{
        Err(String::from("No se pudo abrir el archivo"))
    }

}

fn convertir_lineas_a_vector(un_archivo: File) -> Vec<String>{

    let mut buffer = BufReader::new(un_archivo);

    let mut lineas: Vec<String> = Vec::new();

    procesar_texto(&mut buffer, &mut lineas);

    lineas
}


fn procesar_texto(buffer: &mut BufReader<File>, lineas: &mut Vec<String>) {
    loop {

        let mut linea_actual = String::new();

        match buffer.read_line(&mut linea_actual){
            Ok(0) => {
                //Termina la lectura
                break;
            },
            Ok(_n) => {
                //Agrego la linea leida al vector y restoro la linea_actual
                if linea_actual.ends_with('\n'){
                    linea_actual.pop();
                }
                lineas.push(linea_actual);
            },
            Err(error) => {
                //Termino la lectura e imprimo el mensaje de error
                println!("Ocurrio un error al leer el archivo: {}", error);
                break;
            },
        }
    }
}