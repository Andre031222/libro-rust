//! Generador del arte de la portada: un programa en Rust que dibuja
//! la portada del libro de Rust. Produce SVG sin texto (los títulos
//! se componen después en Canva). El resultado depende solo de la
//! semilla, así que cualquier portada se puede regenerar idéntica.

use std::fmt::Write;

/// Ancho y alto del lienzo: proporción 17 × 24 cm.
pub const ANCHO: f64 = 1700.0;
pub const ALTO: f64 = 2400.0;

const OXIDO: &str = "#B7410E";
const COBRE: &str = "#D27D2D";
const CARBON: &str = "#2B2B2B";
const AZUL: &str = "#1F5F8B";
const ARENA: &str = "#F4EDE4";

/// Motivos disponibles para el arte central.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motivo {
    /// Nodos de propiedad unidos por flechas de préstamo.
    Grafo,
    /// Capas de manchas que imitan óxido sobre metal.
    Oxido,
    /// Hexágonos recursivos, como un caparazón.
    Fractal,
}

/// Generador pseudoaleatorio xorshift64: determinista y sin
/// dependencias externas.
struct Azar(u64);

impl Azar {
    fn new(semilla: u64) -> Self {
        // Se mezcla la semilla con splitmix64 para que semillas
        // vecinas den estados muy distintos; xorshift no admite 0.
        let mut z = semilla.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        Azar(if z == 0 { 1 } else { z })
    }

    fn siguiente(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Número uniforme en [a, b).
    fn rango(&mut self, a: f64, b: f64) -> f64 {
        let u = (self.siguiente() >> 11) as f64 / (1u64 << 53) as f64;
        a + u * (b - a)
    }

    fn elegir<'a>(&mut self, xs: &[&'a str]) -> &'a str {
        xs[self.siguiente() as usize % xs.len()]
    }
}

/// Genera el SVG completo de la portada.
pub fn generar(semilla: u64, motivo: Motivo) -> String {
    let mut azar = Azar::new(semilla);
    let mut svg = String::new();
    let _ = write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {ANCHO} {ALTO}" width="{ANCHO}" height="{ALTO}">"#
    );
    let _ = write!(
        svg,
        r#"<rect width="{ANCHO}" height="{ALTO}" fill="{CARBON}"/>"#
    );
    match motivo {
        Motivo::Grafo => grafo(&mut svg, &mut azar),
        Motivo::Oxido => oxido(&mut svg, &mut azar),
        Motivo::Fractal => fractal(&mut svg, &mut azar),
    }
    svg.push_str("</svg>\n");
    svg
}

fn grafo(svg: &mut String, azar: &mut Azar) {
    let n = 46;
    let mut nodos: Vec<(f64, f64, f64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let x = azar.rango(120.0, ANCHO - 120.0);
        let y = azar.rango(420.0, ALTO - 260.0);
        let r = azar.rango(10.0, 46.0);
        nodos.push((x, y, r));
    }
    // Cada nodo (salvo el primero) tiene un único dueño anterior:
    // un árbol, como la propiedad en Rust.
    for i in 1..n {
        let j = azar.siguiente() as usize % i;
        let (x1, y1, _) = nodos[j];
        let (x2, y2, _) = nodos[i];
        let ancho = azar.rango(3.0, 9.0);
        let _ = write!(
            svg,
            r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="{OXIDO}" stroke-width="{ancho:.1}" stroke-linecap="round"/>"#
        );
    }
    // Préstamos: aristas punteadas que no crean dueños nuevos.
    for _ in 0..n / 2 {
        let a = nodos[azar.siguiente() as usize % n];
        let b = nodos[azar.siguiente() as usize % n];
        let _ = write!(
            svg,
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{AZUL}" stroke-width="3" stroke-dasharray="14 10" opacity="0.8"/>"#,
            a.0, a.1, b.0, b.1
        );
    }
    for (x, y, r) in nodos {
        let color = azar.elegir(&[OXIDO, COBRE, ARENA]);
        let _ = write!(
            svg,
            r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="{color}" stroke="{CARBON}" stroke-width="5"/>"#
        );
    }
}

fn oxido(svg: &mut String, azar: &mut Azar) {
    for _ in 0..900 {
        let x = azar.rango(-200.0, ANCHO + 200.0);
        let y = azar.rango(-200.0, ALTO + 200.0);
        let r = azar.rango(8.0, 260.0);
        let o = azar.rango(0.04, 0.28);
        let color = azar.elegir(&[OXIDO, OXIDO, COBRE, ARENA, AZUL]);
        let _ = write!(
            svg,
            r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r:.1}" fill="{color}" opacity="{o:.2}"/>"#
        );
    }
    // Rayas finas, como metal cepillado.
    for _ in 0..140 {
        let y = azar.rango(0.0, ALTO);
        let largo = azar.rango(200.0, 1100.0);
        let x = azar.rango(-100.0, ANCHO);
        let o = azar.rango(0.05, 0.25);
        let _ = write!(
            svg,
            r#"<line x1="{x:.1}" y1="{y:.1}" x2="{:.1}" y2="{y:.1}" stroke="{ARENA}" stroke-width="1.5" opacity="{o:.2}"/>"#,
            x + largo
        );
    }
}

fn fractal(svg: &mut String, azar: &mut Azar) {
    hexagono(svg, azar, ANCHO / 2.0, ALTO * 0.55, 620.0, 4);
}

fn hexagono(
    svg: &mut String,
    azar: &mut Azar,
    cx: f64,
    cy: f64,
    r: f64,
    nivel: u32,
) {
    let mut puntos = String::new();
    for k in 0..6 {
        let a = std::f64::consts::PI / 3.0 * k as f64;
        let _ = write!(
            puntos,
            "{:.1},{:.1} ",
            cx + r * a.cos(),
            cy + r * a.sin()
        );
    }
    let color = azar.elegir(&[OXIDO, COBRE, AZUL, ARENA]);
    let o = azar.rango(0.35, 0.95);
    let _ = write!(
        svg,
        r#"<polygon points="{}" fill="{color}" fill-opacity="{o:.2}" stroke="{CARBON}" stroke-width="{:.1}"/>"#,
        puntos.trim_end(),
        (r / 40.0).max(1.5)
    );
    if nivel == 0 {
        return;
    }
    let hijo = r / 2.6;
    for k in 0..6 {
        // Algunas ramas se omiten al azar: cada semilla da un
        // caparazón distinto.
        if azar.rango(0.0, 1.0) < 0.22 {
            continue;
        }
        let a = std::f64::consts::PI / 3.0 * k as f64
            + std::f64::consts::PI / 6.0;
        let d = r * 0.62;
        let (x, y) = (cx + d * a.cos(), cy + d * a.sin());
        hexagono(svg, azar, x, y, hijo, nivel - 1);
    }
}
