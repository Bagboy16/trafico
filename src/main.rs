use std::fs::File;
use std::io::BufReader;
use std::io::Read;

struct Vehiculo {
    placa: String,
    cuadrante: i32,
    velocidad: f32,
    infractor: bool,
    multa: f32,
}

fn calculo_multa(velocidad: f32, limite: f32) -> f32 {
    10.0 * (velocidad - limite)
}

fn main() {
    // Open a file, beware of errors
    let file = match File::open("src/reporte.txt") {
        Ok(file) => file,
        Err(why) => {
            panic!("Problem opening the file: {:?}", why)
        }
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    if let Err(why) = buf_reader.read_to_string(&mut contents) {
        panic!("Problem reading the file: {:?}", why)
    }
    let mut vehiculos = Vec::new();
    //Suponiendo que el archivo tiene el formato placa cuadrante velocidad y separando vehiculos por comas
    for x in contents.split(",") {
        let parts = x.split_whitespace().collect::<Vec<&str>>();
        let placa = parts[0].to_string();
        let cuadrante = match parts[1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Error parsing cuadrante for placa: {}", placa);
                continue;
            }
        };
        let velocidad = match parts[2].parse::<f32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Error parsing velocidad for placa: {}", placa);
                continue;
            }
        };
        vehiculos.push(Vehiculo {
            placa,
            cuadrante,
            velocidad,
            infractor: false,
            multa: 0.0,
        });
    }
    let mut vehiculos_antes_de_infraccion = 0;

    let mut primer_infractor = false;

    let mut vehiculos_cuadrante_4_sin_infraccion = 0;
    let mut vehiculos_cuadrante_4_con_infraccion = 0;
    let mut vehiculos_cuadrante_3_con_infraccion = 0;
    let mut vehiculos_cuadrante_2_con_infraccion = 0;
    let mut vehiculos_cuadrante_1_con_infraccion = 0;
    let mut vehiculos_cuadrante_4 = 0;
    let mut cuadrante_con_mas_infractores = 0;
    let mut cantidad_infractores = 0;
    for vehiculo in vehiculos.iter_mut() {
        //Calcular limite de velocidad, la velocidad maxima en los cuadrantes 1 y 2 es 80, en los cuadrantes 3 y 4 es 60. Debe convertirse a Millas por hora, 1 km/h = 0.621371 mph
        let limite_velocidad = match vehiculo.cuadrante {
            1 | 2 => 80.0 * 0.621371,
            3 | 4 => 60.0 * 0.621371,
            _ => {
                println!("Error cuadrante no valido para placa: {}", vehiculo.placa);
                continue;
            }
        };
        if vehiculo.velocidad > limite_velocidad {
            vehiculo.infractor = true;
            vehiculo.multa = calculo_multa(vehiculo.velocidad, limite_velocidad);
            if !primer_infractor {
                primer_infractor = true;
            }
            match vehiculo.cuadrante {
                1 => vehiculos_cuadrante_1_con_infraccion += 1,
                2 => vehiculos_cuadrante_2_con_infraccion += 1,
                3 => vehiculos_cuadrante_3_con_infraccion += 1,
                4 => {
                    vehiculos_cuadrante_4_con_infraccion += 1;
                    vehiculos_cuadrante_4 += 1;
                }
                _ => {}
            }
            if vehiculos_cuadrante_4_con_infraccion > cantidad_infractores {
                cantidad_infractores = vehiculos_cuadrante_4_con_infraccion;
                cuadrante_con_mas_infractores = 4;
            }
            if vehiculos_cuadrante_3_con_infraccion > cantidad_infractores {
                cantidad_infractores = vehiculos_cuadrante_3_con_infraccion;
                cuadrante_con_mas_infractores = 3;
            }
            if vehiculos_cuadrante_2_con_infraccion > cantidad_infractores {
                cantidad_infractores = vehiculos_cuadrante_2_con_infraccion;
                cuadrante_con_mas_infractores = 2;
            }
            if vehiculos_cuadrante_1_con_infraccion > cantidad_infractores {
                cantidad_infractores = vehiculos_cuadrante_1_con_infraccion;
                cuadrante_con_mas_infractores = 1;
            };
        } else {
            if !primer_infractor {
                vehiculos_antes_de_infraccion += 1;
            }
            match vehiculo.cuadrante {
                4 => {
                    vehiculos_cuadrante_4_sin_infraccion += 1;
                    vehiculos_cuadrante_4 += 1;
                }
                _ => {}
            }
        }

        //Imprimr vehiculos
        println!("---------------------------------");
        println!("Placa: {}", vehiculo.placa);
        println!("Cuadrante: {}", vehiculo.cuadrante);
        println!("Velocidad: {} Mph", vehiculo.velocidad);
        if vehiculo.infractor {
            println!("Infractor: SI");
            println!("Multa: {} $", vehiculo.multa);
        }
    }
    println!("---------------------------------");
    //porcentaje de vehiculos que no cometieron infraccion en el cuadrante 4 con respecto al total de vehiculos en el cuadrante 4
    println!("Porcentaje de vehiculos que no cometieron infraccion en el cuadrante 4 con respecto al total de vehiculos en el cuadrante 4: {}%", (vehiculos_cuadrante_4_sin_infraccion as f32 / vehiculos_cuadrante_4 as f32) * 100.0);
    println!(
        "Vehiculos antes de primer infractor: {}",
        vehiculos_antes_de_infraccion
    );
    //Cuadrante con mas infractores
    println!(
        "Cuadrante con mas infractores: {} con {} infractores",
        cuadrante_con_mas_infractores, cantidad_infractores
    );
    //Promedio de multas
    let mut total_multas = 0.0;
    let mut total_infractores = 0;
    for vehiculo in vehiculos.iter() {
        if vehiculo.infractor {
            total_multas += vehiculo.multa;
            total_infractores += 1;
        }
    }
    println!(
        "Promedio de multas: {}",
        total_multas / total_infractores as f32
    );
}
