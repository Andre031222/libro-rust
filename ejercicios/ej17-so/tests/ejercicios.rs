//! Pruebas de los ejercicios del capítulo 17.
//! Ejecute `cargo test -p ej17-so` hasta que pasen.

use ej17_so::*;

#[test]
fn mapa_asigna_y_libera() {
    let mut m = MapaMarcos::<2>::new(); // 16 marcos
    assert_eq!(m.libres(), 16);
    assert_eq!(m.asignar(), Some(0));
    assert_eq!(m.asignar(), Some(1));
    assert_eq!(m.liberar(0), Ok(()));
    assert_eq!(m.liberar(0), Err(ErrorMarco::DobleLiberacion));
    assert_eq!(m.liberar(16), Err(ErrorMarco::FueraDeRango));
    assert_eq!(m.asignar(), Some(0));
    for esperado in 2..16 {
        assert_eq!(m.asignar(), Some(esperado));
    }
    assert_eq!(m.asignar(), None);
    assert_eq!(m.libres(), 0);
}

#[test]
fn mapa_asigna_contiguos() {
    let mut m = MapaMarcos::<2>::new();
    assert_eq!(m.asignar_contiguos(3), Some(0)); // 0, 1, 2
    assert_eq!(m.asignar(), Some(3));
    assert_eq!(m.liberar(1), Ok(()));
    // El hueco en 1 es de un solo marco: no caben 2 ahí.
    assert_eq!(m.asignar_contiguos(2), Some(4)); // 4 y 5
    assert_eq!(m.asignar_contiguos(6), Some(6)); // 6..12
    assert_eq!(m.asignar_contiguos(5), None);
    assert_eq!(m.asignar_contiguos(4), Some(12)); // 12..16
    assert_eq!(m.libres(), 1);
    assert_eq!(m.asignar_contiguos(0), None);
}

#[test]
fn teclado_descarta_la_mas_antigua() {
    let mut t = ColaTeclado::<3>::new();
    assert!(t.is_empty());
    assert_eq!(t.push(b'a'), None);
    assert_eq!(t.push(b'b'), None);
    assert_eq!(t.push(b'c'), None);
    assert_eq!(t.push(b'd'), Some(b'a'));
    assert_eq!(t.len(), 3);
    assert_eq!(t.pop(), Some(b'b'));
    assert_eq!(t.push(b'e'), None);
    assert_eq!(t.pop(), Some(b'c'));
    assert_eq!(t.pop(), Some(b'd'));
    assert_eq!(t.pop(), Some(b'e'));
    assert_eq!(t.pop(), None);
}

fn elf_de_prueba() -> [u8; 64] {
    let mut b = [0u8; 64];
    b[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    b[4] = 2; // 64 bits
    b[5] = 1; // little-endian
    b[6] = 1; // versión
    b[16..18].copy_from_slice(&2u16.to_le_bytes()); // ET_EXEC
    b[18..20].copy_from_slice(&62u16.to_le_bytes()); // x86-64
    b[24..32].copy_from_slice(&0x20_1120u64.to_le_bytes());
    b
}

#[test]
fn elf_valido() {
    let info = leer_elf(&elf_de_prueba()).unwrap();
    assert_eq!(
        info,
        InfoElf {
            tipo: 2,
            maquina: 62,
            entrada: 0x20_1120
        }
    );
}

#[test]
fn elf_invalidos() {
    let b = elf_de_prueba();
    assert_eq!(leer_elf(&b[..40]), Err(ErrorElf::Corta));
    let mut m = b;
    m[1] = b'X';
    assert_eq!(leer_elf(&m), Err(ErrorElf::NoEsElf));
    let mut m = b;
    m[4] = 1;
    assert_eq!(leer_elf(&m), Err(ErrorElf::NoEs64Bits));
    let mut m = b;
    m[5] = 2;
    assert_eq!(leer_elf(&m), Err(ErrorElf::NoEsLittleEndian));
}

#[test]
fn sistema_de_archivos_basico() {
    let mut fs = SistemaArchivos::<2, 8>::new();
    assert_eq!(fs.crear("hola.txt"), Ok(()));
    assert_eq!(fs.crear("hola.txt"), Err(ErrorFs::YaExiste));
    assert_eq!(fs.leer("hola.txt"), Ok(&b""[..]));
    assert_eq!(fs.escribir("hola.txt", b"rust"), Ok(()));
    assert_eq!(fs.leer("hola.txt"), Ok(&b"rust"[..]));
    assert_eq!(
        fs.escribir("hola.txt", b"rustacea!"),
        Err(ErrorFs::SinEspacio)
    );
    assert_eq!(fs.leer("hola.txt"), Ok(&b"rust"[..]));
    assert_eq!(fs.leer("otro"), Err(ErrorFs::NoExiste));
    assert_eq!(
        fs.crear("un_nombre_muy_largo"),
        Err(ErrorFs::NombreLargo)
    );
    assert_eq!(fs.cuantos(), 1);
}

#[test]
fn sistema_de_archivos_reutiliza_entradas() {
    let mut fs = SistemaArchivos::<2, 4>::new();
    fs.crear("a").unwrap();
    fs.crear("b").unwrap();
    assert_eq!(fs.crear("c"), Err(ErrorFs::SinEntradas));
    assert_eq!(fs.borrar("a"), Ok(()));
    assert_eq!(fs.borrar("a"), Err(ErrorFs::NoExiste));
    assert_eq!(fs.crear("c"), Ok(()));
    fs.escribir("c", b"abcd").unwrap();
    assert_eq!(fs.leer("c"), Ok(&b"abcd"[..]));
    assert_eq!(fs.cuantos(), 2);
}
