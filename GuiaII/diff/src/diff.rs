/*
    calcula el diff en base a dos archivos
*/

use crate::{
    lector::read_file_lines,
    lcs_por_linea::GrillaLineaLCS
};


#[derive(Default)]
pub struct Diff{
    
    grilla: GrillaLineaLCS,
}


impl Diff{
    pub fn new(path1: &str, path2: &str) -> Result<Diff, String>{

        let archivo1 = read_file_lines(path1)?;

        let archivo2 = read_file_lines(path2)?;

        let mut grilla = GrillaLineaLCS::new(archivo1, archivo2);

        grilla.lcs();

        Ok(
            Diff{
                grilla,
            }
        )

    }

    //Aca se que fila y columna son ambas mayores a cero
    fn ambas_validas(&self, fila_actual: usize, columna_actual: usize) -> Option<()>{

        let linea_fila = self.grilla.obtener_linea_fila(fila_actual - 1)?;
        let linea_columna = self.grilla.obtener_linea_columna(columna_actual - 1)?;

        if *linea_fila == *linea_columna{
            self.realizar_diff(fila_actual - 1, columna_actual - 1);
            println!("  {}", linea_fila);
        }else{
            let valor_fila = self.grilla.obtener_valor(fila_actual, columna_actual-1)?;
            let valor_columna = self.grilla.obtener_valor(fila_actual-1, columna_actual)?;
            if valor_fila >= valor_columna{
                self.realizar_diff(fila_actual, columna_actual - 1);
                self.imprimir_por_columna(columna_actual-1);
            }else{
                self.realizar_diff(fila_actual - 1, columna_actual);
                self.imprimir_por_fila(fila_actual-1);
            }
        }

        

        None
    }

    fn imprimir_por_columna(&self, columna_actual: usize){
        if let Some(linea) = self.grilla.obtener_linea_columna(columna_actual){
            println!("> {}", linea);
        }
    }

    fn imprimir_por_fila(&self, fila_actual: usize){
        if let Some(linea) = self.grilla.obtener_linea_fila(fila_actual){
            println!("< {}", linea);
        }
    }

    /*
    Empiezo desde el ultimo indice de los strings
    Osea, cuando le pida un valor a la grilla siempre es mas uno
    */
    fn realizar_diff(&self, fila_actual: usize, columna_actual: usize){
        if fila_actual > 0 && columna_actual > 0{
            //Me fijo si son iguales las lineas
            self.ambas_validas(fila_actual, columna_actual);
        }else if columna_actual > 0{
            //Aca fila = 0
            self.realizar_diff(fila_actual, columna_actual - 1);
            self.imprimir_por_columna(columna_actual);
        }else if fila_actual > 0{
            //Aca columna = 0
            self.realizar_diff(fila_actual - 1, columna_actual);
            self.imprimir_por_fila(fila_actual);
        }else{
            println!("");
        }
    }

    pub fn diff(&self){
        let ( numero_filas, numero_columnas) = self.grilla.obtener_dimensiones();

        self.realizar_diff(numero_filas - 1, numero_columnas - 1);
    }

}