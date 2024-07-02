
use rand::Rng;
use std::{io, any};
use std::io::Error;
//use std::io::Write;
//use std::io::Error;
use std::process;
use std::fs::File;
use std::io::prelude::*;

static mut cartas: Vec<String> = Vec::new(); 
static mut cartasRespaldo: Vec<String> = Vec::new(); 
static mut EstadoJuego: bool = true;
static mut fila1: Vec<String> = Vec::new();
static mut fila2: Vec<String> = Vec::new();
static mut fila3: Vec<String> = Vec::new();
static mut fila4: Vec<String> = Vec::new();
static mut fila5: Vec<String> = Vec::new();
static mut fila6: Vec<String> = Vec::new();
static mut fila7: Vec<String> = Vec::new();
static mut filaTemp1: Vec<String> = Vec::new();
static mut filaTemp2: Vec<String> = Vec::new();
static mut vidas: u32 = 3;
static mut Gano: bool = false;
static mut cartastotal: usize = 0;
static mut puntos: usize = 0;


fn reglas(archivo: &mut File){  
    println!();
    println!("Bienvenido!!");
    println!("Solitario de Piramide");
    println!("Comandos Disponibles:");
    println!("RET - Agarrar carta de la baraja sobrante");
    println!("P - Cambiar Carta Esquina");
    println!("N/n - Juego Nuevo");
    println!("ESC - Salir");
    let mut ReglasEscribir = "Bienvenido!!\nSolitario de Piramide\nComandos Disponibles:\nRET - Agarrar carta de la baraja sobrante\nP - Cambiar Carta Esquina\nN/n - Juego Nuevo\nESC - Salir\n";
    escribirArchivo(archivo, ReglasEscribir);
}

fn salirJuego(archivo: &mut File){
    unsafe {
        EstadoJuego = false;
        if Gano == true {
            println!("");
            println!("*--------------------------------------*");
            println!("FELICIDADES!! GANASTE LA PARTIDA");
            println!("Vidas sobrantes: {:?}", vidas);
            println!("*--------------------------------------*");
            println!("");
            EstadoJuego = false;
            let mut ReglasEscribir1 = "\n*--------------------------------------*\nFELICIDADES!! GANASTE LA PARTIDA\nVidas sobrantes:".to_owned() + &vidas.to_string() + "\n*--------------------------------------*";
            escribirArchivo(archivo, &ReglasEscribir1);
        }
        if vidas == 0 {
            println!("");
            println!("*--------------------------------------*");
            println!("Vidas disponibles: {:?}", vidas);
            println!("Vidas disponibles agotadas");
            println!("Perdiste la partida");
            println!("*--------------------------------------*");
            println!("");
            let mut ReglasEscribir1 = "\n*--------------------------------------*\nVidas disponibles:".to_owned() + &vidas.to_string() + "\nVidas disponibles agotadas\nPerdiste la partida\n*--------------------------------------*";
            escribirArchivo(archivo, &ReglasEscribir1);
            process::exit(0);
        }
        if vidas != 0 && Gano != true {
            println!("");
            println!("*--------------------------------------*");
            println!("Saliendo del Juego Solitario de Piramide");
            println!("*--------------------------------------*");
            println!("");
            let mut ReglasEscribir1 = "\n*--------------------------------------*\nSaliendo del Juego Solitario de Piramide:\n*--------------------------------------*";
            escribirArchivo(archivo, &ReglasEscribir1);
            process::exit(0);
        } 
    }

}
fn generarCartas() {
    let cartastipo = ["Cr", "Tn", "Dr", "En"];
    let cartavalores = [
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    unsafe {
        for tipo in cartastipo {
            for valor in cartavalores {
                cartas.push(valor.to_owned() + tipo);
                
            }
        }
    }
}

fn generarTablero() {
    let mut rng = rand::thread_rng();
    unsafe {
        for i in 0..1 {
            let num = rng.gen_range(0..cartas.len());
            fila1.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..2 {
            let num = rng.gen_range(0..cartas.len());
            fila2.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..3 {
            let num = rng.gen_range(0..cartas.len());
            fila3.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..4 {
            let num = rng.gen_range(0..cartas.len());
            fila4.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..5 {
            let num = rng.gen_range(0..cartas.len());
            fila5.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..6 {
            let num = rng.gen_range(0..cartas.len());
            fila6.push(cartas[num].to_string());
            cartas.remove(num);
        }
        for i in 0..7 {
            let num = rng.gen_range(0..cartas.len());
            fila7.push(cartas[num].to_string());
            cartas.remove(num);
        }
    cartastotal = cartas.len();
    }
}

fn revisandoFilaDisponible(filaElegida: usize) -> bool {

    let mut filadisponible = false;
    unsafe {
        if filaElegida == 7 {
            let mut totalcartasvector = 0;
            for element in &fila7 {
                if element != &"   " {
                    totalcartasvector += 1;
                } 
            }
            if totalcartasvector == 0 {
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();   
            } else  {
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!(); 
            }
        }
        if filaElegida == 6 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila7 {
                if element != &"   " {
                    totalcartasvector += 1;
                } 
            }
            for element in &fila6 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            if totalcartasvector < 7 && totalcartasfila != 0{ 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
        if filaElegida == 5 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila6 {
                if element != &"   " {
                    totalcartasvector += 1;
                }
            }
            for element in &fila5 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            if totalcartasvector < 6 && totalcartasfila != 0 { 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
        if filaElegida == 4 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila5 {
                if element != &"   " {
                    totalcartasvector += 1;
                }
            }
            for element in &fila4 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            if totalcartasvector < 5 && totalcartasfila != 0 { 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
        if filaElegida == 3 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila4 {
                if element != &"   " {
                    totalcartasvector += 1;
                }
            }
            for element in &fila3 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            
            if totalcartasvector < 4 && totalcartasfila != 0 { 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
        if filaElegida == 2 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila3 {
                if element != &"   " {
                    totalcartasvector += 1;
                }
            }
            for element in &fila2 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            if totalcartasvector < 3 && totalcartasfila != 0{ 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
        if filaElegida == 1 {
            let mut totalcartasvector = 0;
            let mut totalcartasfila = 0;
            for element in &fila2 {
                if element != &"   " {
                    totalcartasvector += 1;
                }
            }
            for element in &fila1 {
                if element != &"   " {
                    totalcartasfila += 1;
                } 
            }
            if totalcartasvector < 2 && totalcartasfila != 0{ 
                filadisponible = true;
                println!(); 
                print!("Fila disponible");
                println!();
            } else { 
                filadisponible = false;
                println!(); 
                print!("Fila no disponible");
                println!();
            }  
        }
    }
    return filadisponible

}

fn revisandoCartaDisponible(filaElegida: &mut Vec<String>, filaAbajo: &mut Vec<String>, columnaElegida: usize) -> bool {
    let mut cartaDisponible = false;
    let  lendfilaElegida = filaElegida.len();

    if lendfilaElegida == 7 {
        let cartaMover = &filaElegida[columnaElegida];
            if cartaMover != "   " {
                cartaDisponible = true; 
                println!(); 
                print!("Carta disponible");
                println!();
            }
            else {
                cartaDisponible = false; 
                println!(); 
                print!("Carta no disponible");
                println!();
            }      
    } else {
        let cartaAbajoIzq = &filaAbajo[columnaElegida];
        let cartaAbajoDer = &filaAbajo[columnaElegida+1];

        if cartaAbajoIzq != "   " && cartaAbajoDer != "   " {
            cartaDisponible = false;
            println!(); 
            print!("Carta no disponible");
            println!();
        } else  {
            cartaDisponible = true;
            println!(); 
            print!("Carta disponible");
            println!();
        }
    }
    return cartaDisponible;
}

fn movimiento(fila1Elegida: &mut Vec<String>,fila2Elegida: &mut Vec<String>, posicioncarta1: usize, posicioncarta2: usize) -> bool {
    let carta1 = &fila1Elegida[posicioncarta1];
    let carta2 = &fila2Elegida[posicioncarta2];
    let mut numCarta1;
    let mut numCarta2;
    let mut verificando = false;

    if carta1.len() == 3{
        numCarta1 = &carta1[0..1];
    } else {
        numCarta1 = &carta1[0..2];
    }
    if carta2.len() == 3{
        numCarta2 = &carta2[0..1];
    } else {
        numCarta2 = &carta2[0..2];
    }
    if numCarta1 == "J" {
        numCarta1 = "11"
    } 
    if numCarta1 == "Q" {
        numCarta1 = "12"
    }
    if numCarta1 == "K" {
        numCarta1 = "13"
    }
    if numCarta1 == "A" {
        numCarta1 = "1"
    }
    if numCarta2 == "J" {
        numCarta2 = "11"
    } 
    if numCarta2 == "Q" {
        numCarta2 = "12"
    }
    if numCarta2 == "K" {
        numCarta2 = "13"
    }
    if numCarta2 == "A" {
        numCarta2 = "1"
    }
    if numCarta1 == "13" || numCarta2 == "13" {
        if numCarta1 == "13" { 
            &mut fila1Elegida.remove(posicioncarta1);
            &mut fila1Elegida.insert(posicioncarta1, String::from("   "));
            verificando = true;
            unsafe {
                puntos += 10;
            }
        } else {
            &mut fila2Elegida.remove(posicioncarta2);
            &mut fila2Elegida.insert(posicioncarta2,  String::from("   "));
            verificando = true;
            unsafe {
                puntos += 10;
            }
        }   
    } else {
        if numCarta1.parse::<i32>().unwrap()+numCarta2.parse::<i32>().unwrap() == 13{// Revisar que sumen 13
            &mut fila1Elegida.remove(posicioncarta1);
            &mut fila1Elegida.insert(posicioncarta1, String::from("   "));

            &mut fila2Elegida.remove(posicioncarta2);
            &mut fila2Elegida.insert(posicioncarta2,  String::from("   "));
            verificando = true;
            unsafe {
                puntos += 10;
            }
            
         }
    }
    return verificando;
        
    
}

fn imprimirTablero(archivo: &mut File){
    unsafe {
        println!();
        println!("-------------------------------------------------------------");
        println!("Puntos ganados: {:?}", puntos);
        println!("Vueltas a la baraja disponibles: {:?}", vidas);
        println!("Cartas baraja total: {:?}", cartastotal);
        println!("Cartas baraja: {:?}", cartas.len());
        println!("Cartas Esquina: {:?}", cartas[cartas.len()-1]);
        println!("Fila 1:                    {:?}", fila1);
        println!("Fila 2:                {:?}", fila2);
        println!("Fila 3:             {:?}", fila3);
        println!("Fila 4:          {:?}", fila4);
        println!("Fila 5:       {:?}", fila5);
        println!("Fila 6:    {:?}", fila6);
        println!("Fila 7: {:?}", fila7);

        let mut string0 = "\n";
        let mut string14 = "\n-------------------------------------------------------------\n";
        let mut string13 = "\nPuntos ganados: ".to_owned() + &puntos.to_string();
        let mut string1 = "\nVueltas a la baraja disponibles: ".to_owned() + &vidas.to_string();
        let mut string2 = "\nCartas baraja total:".to_owned() + &cartastotal.to_string();
        let mut string3 = "\nCartas baraja:".to_owned() + &cartas.len().to_string();
        let mut string4 = "\nCartas Esquina:".to_owned() + &cartas[cartas.len()-1].to_string();
        let mut string5 = "\nFila 1:               ".to_owned() + &fila1.join(" ");
        let mut string6 = "\nFila 2:             ".to_owned() + &fila2.join(" ");
        let mut string7 = "\nFila 3:           ".to_owned() + &fila3.join(" ");
        let mut string8 = "\nFila 4:         ".to_owned() + &fila4.join(" ");
        let mut string9 = "\nFila 5:       ".to_owned() + &fila5.join(" ");
        let mut string10 = "\nFila 6:    ".to_owned() + &fila6.join(" ");
        let mut string11 = "\nFila 7:  ".to_owned() + &fila7.join(" ");
        let mut string12 = "\n";

        let mut escribirTablero1 = string0.to_owned() + &string14 + &string13 + &string1 + &string2 + &string3 + &string4 + &string5 + &string6 + &string7 + &string8 + &string9 + &string10 + &string11 + &string12;
        escribirArchivo(archivo, &escribirTablero1);
    }
    
}

fn verificandoEsNumero(string: &str) -> bool {
    for i in string.chars() {
        if i.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn revisarGano(){
    // que todos lo que tienen los vectores sea igual a "   "
    let mut revisandofila7 = true;
    let mut revisandofila6 = true;
    let mut revisandofila5 = true;
    let mut revisandofila4 = true;
    let mut revisandofila3 = true;
    let mut revisandofila2 = true;
    let mut revisandofila1 = true;
    let mut respuesta = false;
    unsafe {
        for element in &fila7 {
            if element != &"   " {
                revisandofila7 = false;
            }      
        }
        for element in &fila6 {
            if element != &"   " {
                revisandofila6 = false;
            }      
        }
        for element in &fila5 {
            if element != &"   " {
                revisandofila5 = false;
            }      
        }
        for element in &fila4 {
            if element != &"   " {
                revisandofila4 = false;
            }      
        }
        for element in &fila3 {
            if element != &"   " {
                revisandofila3 = false;
            }      
        }
        for element in &fila2 {
            if element != &"   " {
                revisandofila2 = false;
            }      
        }
        for element in &fila1 {
            if element != &"   " {
                revisandofila1 = false;
            }      
        }
        if  revisandofila7 == true && revisandofila6 == true && revisandofila5 == true && revisandofila4 == true && revisandofila3 == true && revisandofila2 == true && revisandofila1 == true {
            Gano = true;
        } else {
            Gano = false;
        }
    }
}

fn reiniciandotablero() {
    unsafe {
        cartas.clear();
        cartasRespaldo.clear();
        fila7.clear();  
        fila6.clear();  
        fila5.clear();  
        fila4.clear();  
        fila3.clear();  
        fila2.clear();  
        fila1.clear();
        vidas = 3;  
        cartastotal = 0;
        Gano = false;
        EstadoJuego == false;
    }
}

fn crearArchivo() -> File {
    let archivo = File::create("PartidaGuardada.txt").unwrap();
    return archivo; 
}
fn escribirArchivo(file: &mut File, escribir: &str) {
    file.write_all(escribir.as_bytes()).unwrap();
}


fn main() {
    let mut archivo = crearArchivo();
    reglas(&mut archivo);
    generarCartas();
    generarTablero();
    imprimirTablero(&mut archivo);
    unsafe {
        
        while EstadoJuego == true {
            revisarGano();
            if vidas == 0 {
                salirJuego(&mut archivo);
                EstadoJuego == false;
            }
            if Gano == true {
                salirJuego(&mut archivo);
            }
            let mut string1 = "\nDigite el comando, o la primer fila (1 a 7):\n";
            escribirArchivo(&mut archivo, &string1);
            println!("Digite el comando, o la primer fila (1 a 7): ");
            let mut opcionFila1: String = String::new();
            io::stdin()
                .read_line(&mut opcionFila1);
            let opcionFila1Elegida = opcionFila1.trim();
            escribirArchivo(&mut archivo, &opcionFila1Elegida);
            let mut opcionFila1ElegidaesNumero = false;
            opcionFila1ElegidaesNumero = verificandoEsNumero(opcionFila1Elegida);
            if opcionFila1ElegidaesNumero == true && opcionFila1Elegida.parse::<i32>().unwrap() < 8 && opcionFila1Elegida.parse::<i32>().unwrap() >= 0 {
                let mut filaElegida1Disponible = false;
                filaElegida1Disponible = revisandoFilaDisponible(opcionFila1Elegida.parse::<i32>().unwrap() as usize);
                if filaElegida1Disponible == true {
                    let mut string1 = "\nFila disponible\n";
                    escribirArchivo(&mut archivo, &string1);
                    let mut lenFila = 0;
                    if opcionFila1Elegida == "7"{
                        lenFila = fila7.len();
                    }
                    if opcionFila1Elegida == "6"{
                        lenFila = fila6.len();
                    }
                    if opcionFila1Elegida == "5"{
                        lenFila = fila5.len();
                    }
                    if opcionFila1Elegida == "4"{
                        lenFila = fila4.len();
                    }
                    if opcionFila1Elegida == "3"{
                        lenFila = fila3.len();
                    }
                    if opcionFila1Elegida == "2"{
                        lenFila = fila2.len();
                    }
                    if opcionFila1Elegida == "1"{
                        lenFila = fila1.len();
                    }                
                    println!("Digite la posición de la primer carta a utilizar:");
                    let mut string1 = "\nDigite la posición de la primer carta a utilizar:\n";
                    escribirArchivo(&mut archivo, &string1);
                    let mut opcionCarta1: String = String::new();
                    io::stdin()
                        .read_line(&mut opcionCarta1);
                    let opcionCarta1Elegida = opcionCarta1.trim();
                    escribirArchivo(&mut archivo, &opcionCarta1Elegida);
                    let mut opcionCarta1ElegidaesNumero = false;
                    opcionCarta1ElegidaesNumero = verificandoEsNumero(opcionCarta1Elegida);
                    if opcionCarta1ElegidaesNumero == true && opcionCarta1Elegida.parse::<i32>().unwrap() < lenFila as i32 && opcionCarta1Elegida.parse::<i32>().unwrap() >= 0 {
                        let mut CartaElegida1Disponible = false;
                        if opcionFila1Elegida == "7"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila7,&mut fila7, opcionCarta1Elegida.parse::<i32>().unwrap() as usize); 
                        }
                        if opcionFila1Elegida == "6"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila6,&mut fila7, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }
                        if opcionFila1Elegida == "5"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila5,&mut fila6, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }
                        if opcionFila1Elegida == "4"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila4,&mut fila5, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }
                        if opcionFila1Elegida == "3"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila3,&mut fila4, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }
                        if opcionFila1Elegida == "2"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila2,&mut fila3, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }
                        if opcionFila1Elegida == "1"{
                            CartaElegida1Disponible = revisandoCartaDisponible(&mut fila1,&mut fila2, opcionCarta1Elegida.parse::<i32>().unwrap() as usize);
                        }        
                        // Segunda fila la cual vamos a combinar con la carta elegida primero
                        if CartaElegida1Disponible == true {
                            let mut string0 = "\nCarta disponible:\n";
                            escribirArchivo(&mut archivo, &string0);
                            println!("Eliga la segunda fila (1 a 7): ");
                            let mut string1 = "\nEliga la segunda fila (1 a 7):\n";
                            escribirArchivo(&mut archivo, &string1);
                            let mut opcionfila2: String = String::new();
                            io::stdin()
                                .read_line(&mut opcionfila2);
                            let opcionfila2Elegida = opcionfila2.trim();
                            escribirArchivo(&mut archivo, &opcionfila2Elegida);
                            let opcionfila2ElegidaesNumero = verificandoEsNumero(opcionfila2Elegida);
                            let mut lenFila2 = 0;
                            if opcionfila2Elegida == "7"{
                                lenFila2 = fila7.len();
                            }
                            if opcionfila2Elegida == "6"{
                                lenFila2 = fila6.len();
                            }
                            if opcionfila2Elegida == "5"{
                                lenFila2 = fila5.len();
                            }
                            if opcionfila2Elegida == "4"{
                                lenFila2 = fila4.len();
                            }
                            if opcionfila2Elegida == "3"{
                                lenFila2 = fila3.len();
                            }
                            if opcionfila2Elegida == "2"{
                                lenFila2 = fila2.len();
                            }
                            if opcionfila2Elegida == "1"{
                                lenFila2 = fila1.len();
                            } 
                            if opcionfila2ElegidaesNumero == true { 
                                let mut filaElegida2Disponible = false;
                                filaElegida2Disponible = revisandoFilaDisponible(opcionfila2Elegida.parse::<i32>().unwrap() as usize);
                                if filaElegida2Disponible == true {
                                    let mut string0 = "\nFila disponible:\n";
                                    let mut string1 = "\nDigite la posición de la segunda carta a utilizar:\n";
                                    escribirArchivo(&mut archivo, &string0);
                                    escribirArchivo(&mut archivo, &string1);
                                    println!("Digite la posición de la segunda carta a utilizar:");
                                    let mut opcionCarta2: String = String::new();
                                    io::stdin()
                                        .read_line(&mut opcionCarta2);
                                    let opcionCarta2Elegida = opcionCarta2.trim();
                                    escribirArchivo(&mut archivo, &opcionCarta2Elegida);
                                    let opcionCarta2ElegidaesNumero = verificandoEsNumero(opcionCarta2Elegida);
                                    if opcionCarta2ElegidaesNumero == true && opcionCarta2Elegida.parse::<i32>().unwrap() < lenFila2 as i32 && opcionCarta1Elegida.parse::<i32>().unwrap() >= 0 {
                                        let mut cartaElegida2Disponible = false;
                                        if opcionfila2Elegida == "7"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila7,&mut fila7, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "6"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila6,&mut fila7, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "5"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila5,&mut fila6, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "4"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila4,&mut fila5, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "3"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila3,&mut fila4, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "2"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila2,&mut fila3, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if opcionfila2Elegida == "1"{
                                            cartaElegida2Disponible = revisandoCartaDisponible(&mut fila1,&mut fila2, opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                        }
                                        if cartaElegida2Disponible == true {
                                            if opcionFila1Elegida == "7" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila7,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila7,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila7,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila7,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila7,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila7,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila7,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                            }
                                            // fla 6
                                            if opcionFila1Elegida == "6" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila6,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila6,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila6,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila6,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila6,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila6,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila6,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                            }
                                            // 5
                                            if opcionFila1Elegida == "5" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila5,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila5,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila5,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila5,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila5,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila5,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila5,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                            }
                                            // 4
                                            if opcionFila1Elegida == "4" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila4,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila4,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila4,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila4,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila4,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila4,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila4,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                            }
                                             // 3
                                            if opcionFila1Elegida == "3" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila3,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila3,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila3,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila3,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila3,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila3,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila3,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                            }
                                            // 2
                                            if opcionFila1Elegida == "2" {
                                                if opcionfila2Elegida == "7" {
                                                    movimiento(&mut fila2,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "6" {
                                                    movimiento(&mut fila2,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "5" {
                                                    movimiento(&mut fila2,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "4" {
                                                    movimiento(&mut fila2,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "3" {
                                                    movimiento(&mut fila2,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "2" {
                                                    movimiento(&mut fila2,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }
                                                if opcionfila2Elegida == "1" {
                                                    movimiento(&mut fila2,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                    imprimirTablero(&mut archivo);
                                                }

                                                // 1
                                                if opcionFila1Elegida == "1" {
                                                    if opcionfila2Elegida == "7" {
                                                        movimiento(&mut fila1,&mut fila7,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "6" {
                                                        movimiento(&mut fila1,&mut fila6,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "5" {
                                                        movimiento(&mut fila1,&mut fila5,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "4" {
                                                        movimiento(&mut fila1,&mut fila4,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "3" {
                                                        movimiento(&mut fila1,&mut fila3,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "2" {
                                                        movimiento(&mut fila1,&mut fila2,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                    if opcionfila2Elegida == "1" {
                                                        movimiento(&mut fila1,&mut fila1,opcionCarta1Elegida.parse::<i32>().unwrap() as usize,opcionCarta2Elegida.parse::<i32>().unwrap() as usize);
                                                        imprimirTablero(&mut archivo);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }       
                            }
                        }
                    }
                    
                }
            }
            if opcionFila1Elegida.to_string() == "RET" { 
                let mut string1 = "\nRET elegida\n";
                let mut string2 = "\nDigite la fila (1 a 7): \n";
                let mut string3 = "\n";
                escribirArchivo(&mut archivo, &string3);
                escribirArchivo(&mut archivo, &string1);
                escribirArchivo(&mut archivo, &string2);
                println!("");
                println!("RET elegida");
                println!("Digite la fila (1 a 7): ");
                let mut opcionFilaRet: String = String::new();
                io::stdin()
                    .read_line(&mut opcionFilaRet);
                let opcionFilaRetElegida = opcionFilaRet.trim();
                escribirArchivo(&mut archivo, &opcionFilaRetElegida);
                let mut opcionFila1RetesNumero = false;
                opcionFila1RetesNumero = verificandoEsNumero(opcionFilaRetElegida);
                if opcionFila1RetesNumero == true {
                    let mut filaElegidaDisponibleRET = false;
                    filaElegidaDisponibleRET = revisandoFilaDisponible(opcionFilaRetElegida.parse::<i32>().unwrap() as usize);
                    if filaElegidaDisponibleRET == true && opcionFilaRetElegida.parse::<i32>().unwrap() < 8 && opcionFilaRetElegida.parse::<i32>().unwrap() >= 0{
                        let mut lenFilaRet = 0;
                        if opcionFilaRetElegida == "7"{
                            lenFilaRet = fila7.len();
                        }
                        if opcionFilaRetElegida == "6"{
                            lenFilaRet = fila6.len();
                        }
                        if opcionFilaRetElegida == "5"{
                            lenFilaRet = fila5.len();
                        }
                        if opcionFilaRetElegida == "4"{
                            lenFilaRet = fila4.len();
                        }
                        if opcionFilaRetElegida == "3"{
                            lenFilaRet = fila3.len();
                        }
                        if opcionFilaRetElegida == "2"{
                            lenFilaRet = fila2.len();
                        }
                        if opcionFilaRetElegida == "1"{
                            lenFilaRet = fila1.len();
                        }
                        let mut string4 = "\nDigite la posición de la carta a utilizar:\n";
                        escribirArchivo(&mut archivo, &string4);
                        println!("Digite la posición de la carta a utilizar:");
                        let mut opcionCarta1Ret: String = String::new();
                        io::stdin()
                            .read_line(&mut opcionCarta1Ret);
                        let opcionCartaElegidaRet: &str = opcionCarta1Ret.trim();
                        escribirArchivo(&mut archivo, &opcionCartaElegidaRet);
                        let mut opcionCartaElegidaRetesNumero = false;
                        opcionCartaElegidaRetesNumero = verificandoEsNumero(opcionCartaElegidaRet);
                        if opcionCartaElegidaRetesNumero == true && opcionCartaElegidaRet.parse::<i32>().unwrap() < lenFilaRet as i32 && opcionCartaElegidaRet.parse::<i32>().unwrap() >= 0 {
                            let mut CartaElegidaDisponible = false;
                            if opcionFilaRetElegida == "7"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila7,&mut fila7, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize); 
                            }
                            if opcionFilaRetElegida == "6"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila6,&mut fila7, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if opcionFilaRetElegida == "5"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila5,&mut fila6, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if opcionFilaRetElegida == "4"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila4,&mut fila5, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if opcionFilaRetElegida == "3"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila3,&mut fila4, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if opcionFilaRetElegida == "2"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila2,&mut fila3, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if opcionFilaRetElegida == "1"{
                                CartaElegidaDisponible = revisandoCartaDisponible(&mut fila1,&mut fila2, opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                            }
                            if CartaElegidaDisponible == true && opcionCartaElegidaRet.parse::<i32>().unwrap() < lenFilaRet as i32 && opcionCartaElegidaRet.parse::<i32>().unwrap() >= 0 {
                                let mut movimientoCorrecto = false;
                                if opcionFilaRetElegida == "7" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila7,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "6" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila6,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "5" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila5,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "4" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila4,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "3" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila3,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "2" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila2,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                                if opcionFilaRetElegida == "1" {
                                    movimientoCorrecto = movimiento(&mut cartas,&mut fila1,cartas.len()-1,opcionCartaElegidaRet.parse::<i32>().unwrap() as usize);
                                    if movimientoCorrecto == true {
                                        cartas.remove(cartas.len()-1);
                                        unsafe {
                                            cartastotal -= 1;
                                        }
                                    }
                                    imprimirTablero(&mut archivo);
                                }
                            }
                        }
                    } 
                }  
                
            }
            // CAMBIAR CARTA  ESQUINA
            if opcionFila1Elegida.to_string() == "P" { 
                if cartas.len() == 1 {
                    let cartaMover = &cartas[cartas.len()-1];
                    cartasRespaldo.insert(cartasRespaldo.len(), cartaMover.to_string());
                    cartas.remove(cartas.len()-1);
                    for i in 0..cartasRespaldo.len() { 
                        cartas.push(cartasRespaldo[cartasRespaldo.len()-1].to_string());
                        cartasRespaldo.remove(cartasRespaldo.len()-1);
                    }
                    vidas -= 1;
                } else {
                    let cartaMover = &cartas[cartas.len()-1];
                    cartasRespaldo.insert(cartasRespaldo.len(), cartaMover.to_string());
                    cartas.remove(cartas.len()-1);
                }
                imprimirTablero(&mut archivo);
            }
            if opcionFila1Elegida.to_string() == "N" || opcionFila1Elegida.to_string() == "n"{ 
                let mut string1 = "\n";
                let mut string2 = "\nIniciando un juego Nuevo";
                let mut string3 = "\nComandos Disponibles:";
                let mut string4 = "\nRET - Agarrar Carta de las sobrantes";
                let mut string5 = "\nP - Cambiar Carta Esquina";
                let mut string6 = "\nN/n - Juego Nuevoo";
                let mut string7 = "\nESC - Salir\n";
                let mut stringtodo = string1.to_owned() + &string2 + &string3 + &string4 + &string5 + &string6 + &string7;
                escribirArchivo(&mut archivo, &stringtodo);
                println!();
                println!("Iniciando un juego Nuevo");
                println!("Comandos Disponibles:");
                println!("RET - Agarrar Carta de las sobrantes");
                println!("P - Cambiar Carta Esquina");
                println!("N/n - Juego Nuevo");
                println!("ESC - Salir");
                reiniciandotablero();
                generarCartas();
                generarTablero();
                imprimirTablero(&mut archivo);
            } 
            if opcionFila1Elegida.to_string() == "ESC" { 
                salirJuego(&mut archivo);
            }
        }   
    }    
}









 