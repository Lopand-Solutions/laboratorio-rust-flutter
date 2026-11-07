//! Generador de cuevas para un juego 2D: autómata celular sobre una cuadrícula.
//! Se llama desde Flutter con flutter_rust_bridge (ver el README del laboratorio).

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
    if !(8..=512).contains(&ancho) || !(8..=512).contains(&alto) {
        bail!("ancho y alto van de 8 a 512; llegó {ancho}x{alto}");
    }
    if !(30..=70).contains(&relleno_pct) || pasos > 10 {
        bail!("relleno_pct va de 30 a 70 y pasos de 0 a 10");
    }
    let (w, h) = (ancho as usize, alto as usize);
    let mut azar = Azar(semilla);
    let mut celdas: Vec<u8> = (0..w * h)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            if es_borde(x, y, w, h) || azar.hasta(100) < relleno_pct {
                PARED
            } else {
                SUELO
            }
        })
        .collect();
    for _ in 0..pasos {
        celdas = suavizar(&celdas, w, h);
    }
    dejar_la_region_mas_grande(&mut celdas, w, h);
    Ok(Mapa {
        ancho,
        alto,
        celdas,
    })
}

fn es_borde(x: usize, y: usize, w: usize, h: usize) -> bool {
    x == 0 || y == 0 || x == w - 1 || y == h - 1
}

/// Un paso del autómata: con 5 o más vecinas pared, la celda es pared; con 3 o menos, suelo.
fn suavizar(celdas: &[u8], w: usize, h: usize) -> Vec<u8> {
    let mut nuevas = celdas.to_vec();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if es_borde(x, y, w, h) {
                nuevas[i] = PARED; // el borde siempre es pared: nadie se sale del mapa
                continue;
            }
            let vecinas = vecinas_pared(celdas, w, x, y);
            if vecinas >= 5 {
                nuevas[i] = PARED;
            } else if vecinas <= 3 {
                nuevas[i] = SUELO;
            }
        }
    }
    nuevas
}

/// Cuántas de las 8 celdas alrededor de (x, y) son pared. Solo para celdas que no son borde.
fn vecinas_pared(celdas: &[u8], w: usize, x: usize, y: usize) -> u32 {
    let mut n = 0;
    for vy in y - 1..=y + 1 {
        for vx in x - 1..=x + 1 {
            if (vx, vy) != (x, y) && celdas[vy * w + vx] == PARED {
                n += 1;
            }
        }
    }
    n
}

/// Las zonas de suelo conectadas (arriba, abajo, izquierda, derecha), cada una como lista de índices.
pub(crate) fn regiones(celdas: &[u8], w: usize, h: usize) -> Vec<Vec<usize>> {
    let mut vista = vec![false; celdas.len()];
    let mut todas = Vec::new();
    for inicio in 0..celdas.len() {
        if celdas[inicio] != SUELO || vista[inicio] {
            continue;
        }
        let mut region = Vec::new();
        let mut cola = VecDeque::from([inicio]);
        vista[inicio] = true;
        while let Some(i) = cola.pop_front() {
            region.push(i);
            let (x, y) = (i % w, i / w);
            let vecinas = [
                (x > 0).then(|| i - 1),
                (x + 1 < w).then(|| i + 1),
                (y > 0).then(|| i - w),
                (y + 1 < h).then(|| i + w),
            ];
            for v in vecinas.into_iter().flatten() {
                if celdas[v] == SUELO && !vista[v] {
                    vista[v] = true;
                    cola.push_back(v);
                }
            }
        }
        todas.push(region);
    }
    todas
}

/// Rellena con pared todas las zonas de suelo menos la más grande: así el jugador llega a todo el mapa.
fn dejar_la_region_mas_grande(celdas: &mut [u8], w: usize, h: usize) {
    let mut todas = regiones(celdas, w, h);
    todas.sort_by_key(|r| std::cmp::Reverse(r.len()));
    for region in todas.iter().skip(1) {
        for &i in region {
            celdas[i] = PARED;
        }
    }
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
