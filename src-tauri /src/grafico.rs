extern crate serde_big_array;
use serde::{Serialize, Deserialize};
use crate::usb::get_graph;
use crate::post_procesado::{post_process,TIEMPOENTREMUESTRAS};

#[derive(Serialize, Deserialize)]
pub struct Graph {
    x: Vec<f64>,
    y: Vec<f64>,
    e: String,
}

const MILLIVOLTS_PER_ADCCOUNT: f64 = 3.22265625; 

#[tauri::command]
pub fn grafico() -> Graph {
    let mut parse_failed = false;
    let mut ret = Vec::new();
    for _ in 0..5 {
        let str = match get_graph(){
            Ok(s) => s,
            Err(e) => {
                let e = format!("Error: {:?}", e);
                return Graph{x: Vec::new(), y: Vec::new(), e: e};
            }
        };
        for i in str {
            let parsed_u32 = match i
                .replace(|c: char| !c.is_digit(10), "")
                .parse::<u32>()
            {
                Ok(s) => s,
                Err(_) => {
                    parse_failed = true;
                    break;
                }
            };
            ret.push( parsed_u32 as f64
                * MILLIVOLTS_PER_ADCCOUNT
            );
        }
        if !parse_failed {break;}
    }

    if parse_failed {
        return Graph{x: Vec::new(), y: Vec::new(), e: String::from("Error al obtener datos")};
    }

    // FFT POST PROCESADO
    post_process(&mut ret);

    let eje = (0..ret.len()).map(|x| x as f64 * TIEMPOENTREMUESTRAS).collect();

    Graph{ 
        x: eje,
        y: ret,
        e: String::new(),
    }

}
