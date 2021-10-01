/*
*           a   s   d   g   h   k   o   l   a
*       d   0   0   1   1   1   1   1   1   1
*
*       o   0   0   1   1   1   1   2   2   2
*
*       a   1   1   1   1   1   1   2   2   3 -> Observa que el mayor valor siempre va a quedar en esta
                                                 posicion.
*/
use std::str;
/*
    La GrillaLCS es una grilla que guarda la LCS entre dos strings
*/
pub struct GrillaLCS<'a, 'b>{

    grilla: Vec<Vec<usize>>,

    linea1: &'a str, //Hace de cabeza de columna

    linea2: &'b str//Hace de cabeza de fila

}

impl <'a, 'b> GrillaLCS<'a, 'b>{
    
    pub fn new(linea1: &'a str,linea2: &'b str ) -> GrillaLCS<'a, 'b>{

        let mut grilla = Vec::new();

        construir_grilla(&mut grilla, linea1.len(), linea2.len());

        GrillaLCS{
            grilla,
            linea1,
            linea2
        }

    }

    fn comparar_valor(&self, cordenada_x: usize, cordenada_y: usize, contra: usize) -> bool{

        if let Some(columna) = self.grilla.get(cordenada_x){
            if let Some(valor) = columna.get(cordenada_y){
                *valor == contra
            }else{
                false
            }
        }else{
            false
        }

    }

    fn obtener_valor_mut(&mut self, numero_fila: usize, numero_columna: usize) -> Option<&mut usize>{
        if let Some(columna) = self.grilla.get_mut(numero_columna){
                return columna.get_mut(numero_fila);
        }

        None
    }
    

    fn incrementar(&mut self, numero_fila: usize, numero_columna: usize){

            let a_sumar = devolver_clonado(self.obtener_valor_mut(numero_fila - 1, numero_columna - 1));

            if let Some(valor_a_cambiar) = self.obtener_valor_mut(numero_fila, numero_columna){
                *valor_a_cambiar = a_sumar + 1;
            }
    }

    fn decidir_valor(&mut self, numero_fila: usize, numero_columna: usize){

        let valor_superior = devolver_clonado(self.obtener_valor_mut(numero_fila - 1, numero_columna));
        let valor_izquierdo = devolver_clonado(self.obtener_valor_mut(numero_fila, numero_columna - 1));

        if let Some(valor) = self.obtener_valor_mut(numero_fila, numero_columna){
            if valor_superior > valor_izquierdo{
                *valor = valor_superior;
            }else{
                *valor = valor_izquierdo;
            }
        }

    }

    pub fn lcs(&mut self){

        let (dim_x, dim_y) = self.obtener_dimensiones();

        
        for fila in 0..dim_x{
            for columna in 0..dim_y{
                if comparar_caracteres(self.linea1.get(fila..fila+1), self.linea2.get(columna..columna+1)){
                    self.incrementar(fila+1, columna+1);
                }else{
                    self.decidir_valor(fila+1, columna+1);
                }
            }
        }

    }

    pub fn obtener_dimensiones(&self) -> (usize, usize){
        let x = self.grilla.len();
        let y;
        if let Some(columna) = self.grilla.get(0){
            y = columna.len();
        }else{
            y = 0;
        }

        (x-1, y-1)

    }

    pub fn obtener_valor(&self, numero_fila: usize, numero_columna: usize) -> Option<&usize>{
        if let Some(columna) = self.grilla.get(numero_columna){
            return columna.get(numero_fila);
        }
        None
    }

    pub fn obtener_longitud_lsc(&self) -> usize{

        let a_clonar = self.obtener_valor(self.linea2.len(), self.linea1.len());

        devolver_clon(a_clonar)

    }

    fn primera_letra(&self) -> usize{
        if comparar_caracteres(self.linea1.get(0..1), self.linea2.get(0..1)){
            return 1
        }

        0
    }

    fn avanzar_por_columnas(&self, longitud_actual: &mut usize, lsc: &mut String) -> usize{
        let mut columna = 1;let mut fila = 1;
        let (columna_fin, fila_fin) = self.obtener_dimensiones();
        while columna <= columna_fin && fila <= fila_fin{
            let valor_lsc = devolver_clon(self.obtener_valor(fila, columna));
            if valor_lsc > *longitud_actual{
                agregar_caracter(lsc, self.linea1.get(columna..columna+1));
                *longitud_actual = valor_lsc;
                fila += 1;
            }
            columna += 1;
        }
        
        fila
    }

    fn avanzar_por_filas(&self, longitud_actual: &mut usize, lsc: &mut String, mut fila_actual: usize){
        let (columna_fin, fila_fin) = self.obtener_dimensiones();
        while fila_actual <= fila_fin{
            let valor_lsc = devolver_clon(self.obtener_valor(fila_actual, columna_fin));
            if valor_lsc > *longitud_actual{
                agregar_caracter(lsc, self.linea2.get(fila_actual..fila_actual+1));
            }
            *longitud_actual = valor_lsc;
            fila_actual += 1;
        }
    }

    pub fn obtener_lsc(&self) -> String{
        let mut lsc = String::new();

        let mut longitud = self.primera_letra();

        if longitud > 0{
            if let Some(caracter) = self.linea1.get(0..1){
                lsc.push_str(caracter);
            }
        }
        let mut fila = self.avanzar_por_columnas(&mut longitud, &mut lsc);
        
        self.avanzar_por_filas(&mut longitud, &mut lsc, fila);

        lsc
    }


}

fn agregar_caracter(string: &mut String, caracter: Option<&str>){
    if let Some(valido) = caracter{
        string.push_str(valido);
    }
}

fn construir_grilla(una_grilla: &mut Vec<Vec<usize>>, dim_x: usize, dim_y: usize){

    for _i in 0..dim_x + 1{
        let mut columna: Vec<usize> = Vec::new();
        for _j in 0..dim_y + 1{
            columna.push(0);
        }
        una_grilla.push(columna);
    }

}

fn comparar_caracteres(caracter1: Option<&str>, caracter2: Option<&str>) -> bool{
    if let Some(letra1) = caracter1{
        if let Some(letra2) = caracter2 {
            return *letra1 == *letra2;
        }
    }

    false
}

fn devolver_clonado(a_clonar: Option<&mut usize>) -> usize{
    if let Some(valor) = a_clonar{
        *valor
    }else{
        0
    }
}

fn devolver_clon(a_clonar: Option<&usize>) -> usize{
    if let Some(valor) = a_clonar{
        *valor
    }else{
        0
    }

}


#[cfg(test)]
mod test_grilla{
    use super::GrillaLCS;
    #[test]
    fn creacion_correcta_de_grilla(){
        let un_string = String::from("Cinco");
        let otro_string = String::from("Siete");

        let grilla = GrillaLCS::new(&un_string, &otro_string);

        assert_eq!((5, 5), grilla.obtener_dimensiones());

    }

    #[test]
    fn prueba_de_lsc(){
        let un_string = String::from("abcd");
        let otro_string = String::from("adbc");

        let mut grilla = GrillaLCS::new(&un_string, &otro_string);

        grilla.lcs();

        assert_eq!(3, grilla.obtener_longitud_lsc());
    }

    #[test]
    fn prueba_obtencion_lsc(){
        let un_string = String::from("abcd");
        let otro_string = String::from("adbc");

        let mut grilla = GrillaLCS::new(&un_string, &otro_string);

        grilla.lcs();

        assert_eq!("abc", grilla.obtener_lsc());
    }

}