//! Escena vectorial y sus dos salidas: SVG (para Canva) y PDF (para
//! componer la portada en LaTeX). Ambas dibujan exactamente lo mismo.

use std::collections::BTreeSet;
use std::fmt::Write;

pub const ANCHO: f64 = 1700.0;
pub const ALTO: f64 = 2400.0;

/// Color RGB de 8 bits por canal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const fn hex(h: u32) -> Color {
        Color((h >> 16) as u8, (h >> 8) as u8, h as u8)
    }

    /// Mezcla lineal: t = 0 da `self`, t = 1 da `otro`.
    pub fn mezcla(self, otro: Color, t: f64) -> Color {
        let m = |a: u8, b: u8| {
            (a as f64 + (b as f64 - a as f64) * t.clamp(0.0, 1.0))
                .round() as u8
        };
        Color(
            m(self.0, otro.0),
            m(self.1, otro.1),
            m(self.2, otro.2),
        )
    }

    fn svg(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    fn pdf(self) -> String {
        let c = |v: u8| v as f64 / 255.0;
        format!(
            "{:.3} {:.3} {:.3}",
            c(self.0),
            c(self.1),
            c(self.2)
        )
    }
}

pub enum Forma {
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        relleno: Color,
        op: f64,
    },
    Circulo {
        cx: f64,
        cy: f64,
        r: f64,
        relleno: Color,
        op: f64,
        borde: Option<(Color, f64)>,
    },
    Linea {
        puntos: Vec<(f64, f64)>,
        color: Color,
        ancho: f64,
        op: f64,
        punteada: bool,
    },
    Poligono {
        puntos: Vec<(f64, f64)>,
        relleno: Color,
        op: f64,
        borde: (Color, f64),
    },
}

#[derive(Default)]
pub struct Escena {
    pub formas: Vec<Forma>,
}

impl Escena {
    pub fn svg(&self) -> String {
        let mut s = String::new();
        let _ = write!(
            s,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {ANCHO} {ALTO}" width="{ANCHO}" height="{ALTO}">"#
        );
        for f in &self.formas {
            forma_svg(&mut s, f);
        }
        s.push_str("</svg>\n");
        s
    }

    pub fn pdf(&self) -> Vec<u8> {
        let mut opacidades = BTreeSet::new();
        let mut c = String::from("1 J 1 j\n");
        // Unidades de 0,1 mm con el origen arriba, como en el SVG.
        c.push_str("0.283465 0 0 -0.283465 0 680.315 cm\n");
        for f in &self.formas {
            forma_pdf(&mut c, f, &mut opacidades);
        }
        documento_pdf(&c, &opacidades)
    }
}

fn op_svg(op: f64) -> String {
    if op >= 0.999 {
        String::new()
    } else {
        format!(r#" opacity="{op:.2}""#)
    }
}

fn lista(puntos: &[(f64, f64)]) -> String {
    let mut s = String::new();
    for (x, y) in puntos {
        let _ = write!(s, "{x:.1},{y:.1} ");
    }
    s.trim_end().to_string()
}

fn forma_svg(s: &mut String, f: &Forma) {
    match f {
        Forma::Rect {
            x,
            y,
            w,
            h,
            relleno,
            op,
        } => {
            let _ = write!(
                s,
                r#"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" fill="{}"{}/>"#,
                relleno.svg(),
                op_svg(*op)
            );
        }
        Forma::Circulo {
            cx,
            cy,
            r,
            relleno,
            op,
            borde,
        } => {
            let b = borde
                .map(|(c, w)| {
                    format!(
                        r#" stroke="{}" stroke-width="{w:.1}""#,
                        c.svg()
                    )
                })
                .unwrap_or_default();
            let _ = write!(
                s,
                r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{r:.1}" fill="{}"{}{b}/>"#,
                relleno.svg(),
                op_svg(*op)
            );
        }
        Forma::Linea {
            puntos,
            color,
            ancho,
            op,
            punteada,
        } => {
            let d = if *punteada {
                r#" stroke-dasharray="14 10""#
            } else {
                ""
            };
            let _ = write!(
                s,
                r#"<polyline points="{}" fill="none" stroke="{}" stroke-width="{ancho:.2}" stroke-linecap="round" stroke-linejoin="round"{d}{}/>"#,
                lista(puntos),
                color.svg(),
                op_svg(*op)
            );
        }
        Forma::Poligono {
            puntos,
            relleno,
            op,
            borde,
        } => {
            let _ = write!(
                s,
                r#"<polygon points="{}" fill="{}" fill-opacity="{op:.2}" stroke="{}" stroke-width="{:.1}"/>"#,
                lista(puntos),
                relleno.svg(),
                borde.0.svg(),
                borde.1
            );
        }
    }
}

/// Clave de estado gráfico para una opacidad (centésimas).
fn gs(c: &mut String, op: f64, usadas: &mut BTreeSet<u32>) {
    let k = (op.clamp(0.0, 1.0) * 100.0).round() as u32;
    usadas.insert(k);
    let _ = writeln!(c, "/G{k} gs");
}

fn circulo_pdf(c: &mut String, cx: f64, cy: f64, r: f64) {
    let k = 0.552_284_75 * r;
    let segmentos = [
        [(cx + r, cy + k), (cx + k, cy + r), (cx, cy + r)],
        [(cx - k, cy + r), (cx - r, cy + k), (cx - r, cy)],
        [(cx - r, cy - k), (cx - k, cy - r), (cx, cy - r)],
        [(cx + k, cy - r), (cx + r, cy - k), (cx + r, cy)],
    ];
    let _ = writeln!(c, "{:.1} {cy:.1} m", cx + r);
    for [(x1, y1), (x2, y2), (x3, y3)] in segmentos {
        let _ = writeln!(
            c,
            "{x1:.1} {y1:.1} {x2:.1} {y2:.1} {x3:.1} {y3:.1} c"
        );
    }
}

fn trazo(c: &mut String, puntos: &[(f64, f64)]) {
    for (i, (x, y)) in puntos.iter().enumerate() {
        let op = if i == 0 { "m" } else { "l" };
        let _ = writeln!(c, "{x:.1} {y:.1} {op}");
    }
}

fn forma_pdf(
    c: &mut String,
    f: &Forma,
    usadas: &mut BTreeSet<u32>,
) {
    match f {
        Forma::Rect {
            x,
            y,
            w,
            h,
            relleno,
            op,
        } => {
            gs(c, *op, usadas);
            let _ = writeln!(
                c,
                "{} rg {x:.1} {y:.1} {w:.1} {h:.1} re f",
                relleno.pdf()
            );
        }
        Forma::Circulo {
            cx,
            cy,
            r,
            relleno,
            op,
            borde,
        } => {
            gs(c, *op, usadas);
            let _ = writeln!(c, "{} rg", relleno.pdf());
            circulo_pdf(c, *cx, *cy, *r);
            match borde {
                Some((col, w)) => {
                    let _ = writeln!(
                        c,
                        "{} RG {w:.1} w B",
                        col.pdf()
                    );
                }
                None => c.push_str("f\n"),
            }
        }
        Forma::Linea {
            puntos,
            color,
            ancho,
            op,
            punteada,
        } => {
            gs(c, *op, usadas);
            let d =
                if *punteada { "[14 10] 0 d" } else { "[] 0 d" };
            let _ = writeln!(
                c,
                "{} RG {ancho:.2} w {d}",
                color.pdf()
            );
            trazo(c, puntos);
            c.push_str("S\n");
        }
        Forma::Poligono {
            puntos,
            relleno,
            op,
            borde,
        } => {
            gs(c, *op, usadas);
            let _ = writeln!(
                c,
                "{} rg {} RG {:.1} w [] 0 d",
                relleno.pdf(),
                borde.0.pdf(),
                borde.1
            );
            trazo(c, puntos);
            c.push_str("h B\n");
        }
    }
}

/// Arma un PDF 1.4 mínimo de una página con offsets xref exactos.
fn documento_pdf(
    contenido: &str,
    opacidades: &BTreeSet<u32>,
) -> Vec<u8> {
    let mut estados = String::new();
    for k in opacidades {
        let a = *k as f64 / 100.0;
        let _ =
            write!(estados, "/G{k} << /CA {a:.2} /ca {a:.2} >> ");
    }
    let objetos = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 481.89 680.31] \
             /Resources << /ExtGState << {estados}>> >> /Contents 4 0 R >>"
        ),
        format!(
            "<< /Length {} >>\nstream\n{contenido}endstream",
            contenido.len()
        ),
    ];
    let mut pdf: Vec<u8> =
        b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objetos.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend_from_slice(
            format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes(),
        );
    }
    let xref = pdf.len();
    let mut cola = format!(
        "xref\n0 {}\n0000000000 65535 f \n",
        objetos.len() + 1
    );
    for off in offsets {
        let _ = writeln!(cola, "{off:010} 00000 n ");
    }
    let _ = write!(
        cola,
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
        objetos.len() + 1
    );
    pdf.extend_from_slice(cola.as_bytes());
    pdf
}
