//! Ejercicios del capítulo 8: structs, enums y patrones.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej08-tipos-propios`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 8.4. Un rectángulo de lados enteros.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

impl Rectangulo {
    /// Crea un rectángulo con el ancho y el alto dados.
    pub fn new(ancho: u32, alto: u32) -> Self {
        todo!()
    }

    /// Crea un cuadrado de lado `lado`.
    pub fn cuadrado(lado: u32) -> Self {
        todo!()
    }

    /// Devuelve el área.
    pub fn area(&self) -> u32 {
        todo!()
    }

    /// Indica si `otro` cabe dentro sin girarlo.
    pub fn puede_contener(&self, otro: &Rectangulo) -> bool {
        todo!()
    }

    /// Multiplica ambos lados por `factor`.
    pub fn escalar(&mut self, factor: u32) {
        todo!()
    }
}

/// Ejercicio 8.5. Una figura plana.
#[derive(Debug)]
pub enum Figura {
    Circulo { radio: f64 },
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo { base: f64, altura: f64 },
}

impl Figura {
    /// Devuelve el área de la figura con un `match`.
    pub fn area(&self) -> f64 {
        todo!()
    }
}

/// Ejercicio 8.6. Estados de una puerta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Puerta {
    Abierta,
    Cerrada,
    ConLlave,
}

/// Acciones que se pueden intentar sobre una puerta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accion {
    Abrir,
    Cerrar,
    EcharLlave,
    QuitarLlave,
}

/// Devuelve el estado que sigue a `accion`. Si la acción no tiene
/// sentido en el estado actual, la puerta queda igual.
pub fn transicion(estado: Puerta, accion: Accion) -> Puerta {
    todo!()
}

/// Ejercicio 8.7 (reto). Una expresión aritmética.
#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Suma(Box<Expr>, Box<Expr>),
    Resta(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

/// Evalúa `e`. Devuelve `None` si hay una división entre cero.
pub fn evaluar(e: &Expr) -> Option<f64> {
    todo!()
}
