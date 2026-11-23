//! Capítulo 20: un módulo pensado para WebAssembly.

/// Máximo común divisor por el algoritmo de Euclides.
#[unsafe(no_mangle)]
pub extern "C" fn mcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Indica si `n` es primo, probando divisores hasta su raíz.
#[unsafe(no_mangle)]
pub extern "C" fn es_primo(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let n = u64::from(n);
    (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn calcula_mcd_y_primos() {
        assert_eq!(mcd(84, 36), 12);
        assert_eq!(mcd(7, 0), 7);
        assert!(es_primo(4_294_967_291));
        assert!(!es_primo(1));
    }
}
