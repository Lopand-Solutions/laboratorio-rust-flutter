//! Generador de cuevas para un juego 2D: autómata celular sobre una cuadrícula.
//! Se llama desde Flutter con flutter_rust_bridge (ver el README del laboratorio).
// Rama inicio: borra esta línea cuando termines; solo calla los avisos de lo que todavía no escribes.
#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::VecDeque;

use anyhow::{Result, bail};

pub const PARED: u8 = 1;
pub const SUELO: u8 = 0;

/// Un mapa de `ancho` x `alto` celdas, fila por fila. En Dart, `celdas` llega como `Uint8List`.
pub struct Mapa {
    pub ancho: u32,
    pub alto: u32,
    pub celdas: Vec<u8>,
}

/// Genera una cueva. La misma semilla da siempre el mismo mapa.
///
/// - `relleno_pct`: qué tanto empieza como pared (de 30 a 70).
/// - `pasos`: cuántas veces se suaviza el ruido inicial (de 0 a 10).
pub fn generar_cueva(
    ancho: u32,
    alto: u32,
    semilla: u64,
    relleno_pct: u32,
    pasos: u32,
) -> Result<Mapa> {
    todo!("Paso 2: valida, siembra el ruido, suaviza y deja la región más grande")
}

fn es_borde(x: usize, y: usize, w: usize, h: usize) -> bool {
    x == 0 || y == 0 || x == w - 1 || y == h - 1
}

/// Un paso del autómata: con 5 o más vecinas pared, la celda es pared; con 3 o menos, suelo.
fn suavizar(celdas: &[u8], w: usize, h: usize) -> Vec<u8> {
    todo!("Paso 3: un paso del autómata celular")
}

/// Cuántas de las 8 celdas alrededor de (x, y) son pared. Solo para celdas que no son borde.
fn vecinas_pared(celdas: &[u8], w: usize, x: usize, y: usize) -> u32 {
    todo!("Paso 3: cuenta las 8 vecinas que son pared")
}

/// Las zonas de suelo conectadas (arriba, abajo, izquierda, derecha), cada una como lista de índices.
pub(crate) fn regiones(celdas: &[u8], w: usize, h: usize) -> Vec<Vec<usize>> {
    todo!("Paso 4: búsqueda en anchura de las zonas de suelo")
}

/// Rellena con pared todas las zonas de suelo menos la más grande: así el jugador llega a todo el mapa.
fn dejar_la_region_mas_grande(celdas: &mut [u8], w: usize, h: usize) {
    todo!("Paso 4: rellena con pared todas las zonas menos la más grande")
}

/// SplitMix64: pequeño y repetible.
struct Azar(u64);

impl Azar {
    fn hasta(&mut self, n: u32) -> u32 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (((z ^ (z >> 31)) >> 11) % n as u64) as u32
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn cueva(semilla: u64) -> Mapa {
        generar_cueva(80, 50, semilla, 45, 5).unwrap()
    }

    #[test]
    fn la_misma_semilla_da_el_mismo_mapa() {
        assert_eq!(cueva(7).celdas, cueva(7).celdas);
        assert_ne!(cueva(7).celdas, cueva(8).celdas);
    }

    #[test]
    fn el_mapa_tiene_el_tamano_pedido() {
        let m = cueva(1);
        assert_eq!((m.ancho, m.alto, m.celdas.len()), (80, 50, 80 * 50));
    }

    #[test]
    fn el_borde_es_pared() {
        for semilla in 0..20 {
            let m = cueva(semilla);
            let (w, h) = (m.ancho as usize, m.alto as usize);
            for y in 0..h {
                for x in 0..w {
                    if es_borde(x, y, w, h) {
                        assert_eq!(m.celdas[y * w + x], PARED, "semilla {semilla}, ({x}, {y})");
                    }
                }
            }
        }
    }

    #[test]
    fn todo_el_suelo_esta_conectado() {
        for semilla in 0..20 {
            let m = cueva(semilla);
            let r = regiones(&m.celdas, m.ancho as usize, m.alto as usize);
            assert_eq!(r.len(), 1, "semilla {semilla}: {} zonas de suelo", r.len());
        }
    }

    #[test]
    fn hay_suficiente_suelo_para_jugar() {
        for semilla in 0..20 {
            let m = cueva(semilla);
            let suelo = m.celdas.iter().filter(|&&c| c == SUELO).count();
            assert!(
                suelo * 100 / m.celdas.len() >= 25,
                "semilla {semilla}: {suelo} celdas"
            );
        }
    }

    #[test]
    fn rechaza_parametros_fuera_de_rango() {
        assert!(generar_cueva(4, 50, 1, 45, 5).is_err());
        assert!(generar_cueva(80, 50, 1, 90, 5).is_err());
        assert!(generar_cueva(80, 50, 1, 45, 11).is_err());
    }
}
