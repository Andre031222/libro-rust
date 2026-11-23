//! Los motivos del arte de portada. Cada uno llena una `Escena`.

use crate::azar::{Azar, Ruido};
use crate::escena::{ALTO, ANCHO, Color, Escena, Forma};
use std::f64::consts::{PI, TAU};

pub const OXIDO: Color = Color::hex(0xC83C06);
pub const COBRE: Color = Color::hex(0xE5791A);
pub const CARBON: Color = Color::hex(0x2B2B2B);
pub const AZUL: Color = Color::hex(0x1565A8);
pub const ARENA: Color = Color::hex(0xF4EDE4);
const NOCHE: Color = Color::hex(0x120E0C);

fn fondo(e: &mut Escena, color: Color) {
    e.formas.push(Forma::Rect {
        x: 0.0,
        y: 0.0,
        w: ANCHO,
        h: ALTO,
        relleno: color,
        op: 1.0,
    });
}

/// Degradado vertical aproximado con bandas finas (igual en SVG y PDF).
fn degradado(e: &mut Escena, arriba: Color, abajo: Color) {
    // Base del color final y bandas solapadas: sin costuras visibles.
    fondo(e, abajo);
    let n = 160;
    let h = ALTO / n as f64;
    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        e.formas.push(Forma::Rect {
            x: 0.0,
            y: i as f64 * h - h,
            w: ANCHO,
            h: h * 3.0,
            relleno: arriba.mezcla(abajo, t),
            op: 1.0,
        });
    }
}

/// Resplandor: círculos concéntricos muy tenues.
fn resplandor(
    e: &mut Escena,
    cx: f64,
    cy: f64,
    r: f64,
    color: Color,
) {
    for k in 0..40 {
        let f = 1.0 - k as f64 / 40.0;
        e.formas.push(Forma::Circulo {
            cx,
            cy,
            r: r * f,
            relleno: color,
            op: 0.018,
            borde: None,
        });
    }
}

pub fn grafo(e: &mut Escena, azar: &mut Azar) {
    fondo(e, CARBON);
    let n = 46;
    let nodos: Vec<(f64, f64, f64)> = (0..n)
        .map(|_| {
            (
                azar.rango(120.0, ANCHO - 120.0),
                azar.rango(420.0, ALTO - 260.0),
                azar.rango(10.0, 46.0),
            )
        })
        .collect();
    // Cada nodo (salvo el primero) tiene un único dueño anterior.
    for i in 1..n {
        let j = azar.indice(i);
        e.formas.push(Forma::Linea {
            puntos: vec![
                (nodos[j].0, nodos[j].1),
                (nodos[i].0, nodos[i].1),
            ],
            color: OXIDO,
            ancho: azar.rango(3.0, 9.0),
            op: 1.0,
            punteada: false,
        });
    }
    // Préstamos: aristas punteadas que no crean dueños nuevos.
    for _ in 0..n / 2 {
        let a = nodos[azar.indice(n)];
        let b = nodos[azar.indice(n)];
        e.formas.push(Forma::Linea {
            puntos: vec![(a.0, a.1), (b.0, b.1)],
            color: AZUL,
            ancho: 3.0,
            op: 0.8,
            punteada: true,
        });
    }
    for (x, y, r) in nodos {
        e.formas.push(Forma::Circulo {
            cx: x,
            cy: y,
            r,
            relleno: azar.elegir(&[OXIDO, COBRE, ARENA]),
            op: 1.0,
            borde: Some((CARBON, 5.0)),
        });
    }
}

pub fn oxido(e: &mut Escena, azar: &mut Azar) {
    fondo(e, CARBON);
    for _ in 0..900 {
        e.formas.push(Forma::Circulo {
            cx: azar.rango(-200.0, ANCHO + 200.0),
            cy: azar.rango(-200.0, ALTO + 200.0),
            r: azar.rango(8.0, 260.0),
            relleno: azar
                .elegir(&[OXIDO, OXIDO, COBRE, ARENA, AZUL]),
            op: azar.rango(0.04, 0.28),
            borde: None,
        });
    }
}

pub fn fractal(e: &mut Escena, azar: &mut Azar) {
    fondo(e, CARBON);
    hexagono(e, azar, ANCHO / 2.0, ALTO * 0.55, 620.0, 4);
}

fn hexagono(
    e: &mut Escena,
    azar: &mut Azar,
    cx: f64,
    cy: f64,
    r: f64,
    nivel: u32,
) {
    let puntos = (0..6)
        .map(|k| {
            let a = PI / 3.0 * k as f64;
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect();
    e.formas.push(Forma::Poligono {
        puntos,
        relleno: azar.elegir(&[OXIDO, COBRE, AZUL, ARENA]),
        op: azar.rango(0.35, 0.95),
        borde: (CARBON, (r / 40.0).max(1.5)),
    });
    if nivel == 0 {
        return;
    }
    for k in 0..6 {
        // Algunas ramas se omiten: cada semilla da un caparazón distinto.
        if azar.rango(0.0, 1.0) < 0.22 {
            continue;
        }
        let a = PI / 3.0 * k as f64 + PI / 6.0;
        let d = r * 0.62;
        let (x, y) = (cx + d * a.cos(), cy + d * a.sin());
        hexagono(e, azar, x, y, r / 2.6, nivel - 1);
    }
}

/// Flujo: miles de trazos que siguen un campo de ruido, como óxido
/// que avanza sobre metal. La parte superior queda despejada para el
/// título y el color se calienta hacia abajo.
pub fn flujo(e: &mut Escena, azar: &mut Azar, semilla: u64) {
    degradado(e, NOCHE, CARBON);
    resplandor(e, ANCHO * 0.62, ALTO * 0.70, 1100.0, OXIDO);
    let ruido = Ruido::new(semilla);
    let escala = 1.0 / azar.rango(520.0, 760.0);
    let giro = azar.rango(0.0, TAU);
    let trazos = 2400;
    for _ in 0..trazos {
        let mut x = azar.rango(-100.0, ANCHO + 100.0);
        let mut y = azar.rango(ALTO * 0.30, ALTO + 100.0);
        let pasos = azar.rango(40.0, 150.0) as usize;
        let mut puntos = Vec::with_capacity(pasos);
        for _ in 0..pasos {
            puntos.push((x, y));
            let a =
                ruido.fractal(x * escala, y * escala) * TAU * 2.0
                    + giro;
            x += a.cos() * 6.0;
            y += a.sin() * 6.0;
            if !(-150.0..=ANCHO + 150.0).contains(&x)
                || !(-150.0..=ALTO + 150.0).contains(&y)
            {
                break;
            }
        }
        if puntos.len() < 8 {
            continue;
        }
        // Calor según la altura: arena arriba, óxido profundo abajo.
        let t = (puntos[0].1 / ALTO).clamp(0.0, 1.0);
        let base = if azar.rango(0.0, 1.0) < 0.08 {
            AZUL
        } else if t < 0.55 {
            ARENA.mezcla(COBRE, (t - 0.30) / 0.25)
        } else {
            COBRE.mezcla(OXIDO, (t - 0.55) / 0.45)
        };
        // Se desvanece al acercarse a la zona del título.
        let desvanecer = ((puntos[0].1 - ALTO * 0.30)
            / (ALTO * 0.18))
            .clamp(0.15, 1.0);
        e.formas.push(Forma::Linea {
            puntos,
            color: base,
            ancho: azar.rango(0.8, 3.2),
            op: azar.rango(0.30, 0.85) * desvanecer,
            punteada: false,
        });
    }
}
