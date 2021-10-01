/*
        Realizo el lcs linea a linea en este caso, para construir un diff
*/
#[derive(Default)]
pub struct GrillaLineaLCS{

    grilla: Vec<Vec<usize>>,

    archivo1: Vec<String>, //Son las filas

    archivo2: Vec<String> //son las columnas

}


impl<'a, 'b> GrillaLineaLCS{

    pub fn new(archivo1: Vec<String>, archivo2: Vec<String>) -> GrillaLineaLCS{

        let grilla = construir_grilla(archivo1.len(), archivo2.len());

        GrillaLineaLCS{
            grilla,
            archivo1,
            archivo2
        }


    }

    pub fn obtener_dimensiones(&self) -> (usize, usize){

        let filas = self.grilla.len();
        let mut columnas = 0;

        if let Some(columna) = self.grilla.get(0){
            columnas = columna.len();
        }

        (filas, columnas)
    }

    pub fn obtener_valor(&self, fila: usize, columna: usize) -> Option<usize>{
        if let Some(fila_valida) = self.grilla.get(fila){
            if let Some(valor_valido) = fila_valida.get(columna){
                return Some(valor_valido.clone());
            }
        }

        None
    }

    pub fn obtener_linea_fila(&self, fila: usize) -> Option<&String>{

        self.archivo1.get(fila)
        
    }

    pub fn obtener_linea_columna(&self, columna: usize) -> Option<&String>{
        
        self.archivo2.get(columna)

    }

    pub fn obtener_valor_mut(&mut self, fila: usize, columna: usize) -> Option<&mut usize>{
        if let Some(fila_valida) = self.grilla.get_mut(fila){
            return fila_valida.get_mut(columna);
        }

        None
    }

    fn comparar_lineas(&self, linea1: Option<&String>, linea2: Option<&String>) -> bool{
        if let Some(linea) = linea1{
            if let Some(otra_linea) = linea2{
                return *linea == *otra_linea;
            }
        }

        false
    }

    fn incrementar(&mut self, fila: usize, columna: usize){
        let a_sumar = self.obtener_valor(fila - 1, columna - 1);

        if let Some(valor_a_sumar) = a_sumar{

            let a_incrementar = self.obtener_valor_mut(fila, columna);

            if let Some(valor_a_incrementar) = a_incrementar{
                *valor_a_incrementar = valor_a_sumar + 1;
            }
        }
    }

    fn decidir_valor(&mut self, fila: usize, columna: usize){
        let valor_superior = self.obtener_valor(fila - 1, columna);
        let valor_izquierdo = self.obtener_valor(fila, columna - 1);

        if let Some(valor_valido_izquierdo) = valor_izquierdo{
            if let Some(valor_valido_superior) = valor_superior{
                let valor = self.obtener_valor_mut(fila, columna);
                if let Some(valor_valido) = valor{
                    cambiar_por_mayor(valor_valido, valor_valido_izquierdo, valor_valido_superior);
                }
            }
        }
    }

    pub fn lcs(&mut self){
        let (filas, columnas) = self.obtener_dimensiones();

        for fila in 0..filas{
            for columna in 0..columnas{
                if self.comparar_lineas(self.archivo1.get(fila), self.archivo2.get(columna)){
                    self.incrementar(fila+1, columna+1);
                }else{
                    self.decidir_valor(fila+1, columna+1);
                }
            }
        }
    }

}


fn construir_grilla(numero_lineas1: usize, numero_lineas2: usize) -> Vec<Vec<usize>>{


    let mut grilla: Vec<Vec<usize>> = Vec::new();

    for _fila in 0..numero_lineas1+1 {
        let mut fila: Vec<usize> = Vec::new();
        for _columna in 0..numero_lineas2+1{
            fila.push(0);
        }
        grilla.push(fila);
    }

    grilla
}


fn cambiar_por_mayor(a_cambiar: &mut usize, valor1: usize, valor2: usize){
    if valor1 > valor2{
        *a_cambiar = valor1;
    }else{
        *a_cambiar = valor2;
    }

}
