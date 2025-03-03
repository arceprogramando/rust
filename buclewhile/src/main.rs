fn main() {
    // 🔄 Diferencias entre bucles:
    // - `for`: Se usa cuando conocemos el rango de iteraciones.
    // - `while`: Se usa cuando queremos evaluar una condición en cada iteración.

    let continuar = true; // Variable de control para el bucle principal

    'bucle_principal: while continuar {
        println!("\n🔹 Entrando al Bucle Principal...");

        let mut contador = 0;

        while contador < 10 {
            println!("  🔸 Bucle While → Contador: {}", contador);

            'bucle_interno: for numero in 0..10 {
                if contador == 2 {
                    println!("    🚪 Saliendo del Bucle Interno en contador == 2");
                    break 'bucle_interno;
                }
                println!("    🔹 Bucle For → Número: {}", numero);
            }

            if contador == 5 {
                println!("  🚪 Saliendo del Bucle Principal en contador == 5");
                break 'bucle_principal;
            }

            contador += 1;
        }

        println!("  ✅ Fin del Bucle While");
    }

    println!("✅ Programa Finalizado.");
}
