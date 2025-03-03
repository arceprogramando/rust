fn main() {
    println!("Iniciando el bucle principal...");

    let resultado: i32 = 'bucle_principal: loop {
        for valor_externo in 7..20 {
            for valor_interno in 0..8 {
                println!("Valor Externo: {valor_externo}, Valor Interno: {valor_interno}");

                if valor_externo == 9 && valor_interno == 2 {
                    println!("Condición alcanzada. Saliendo del bucle.");
                    break 'bucle_principal valor_externo + valor_interno;
                }
            }
            println!("Terminó el segundo bucle para el valor externo: {valor_externo}");
        }
        println!("Terminó el primer bucle.");
        break 0;
    };

    println!(
        "Valor final de la suma entre el valor externo e interno : {}",
        resultado
    );
    println!("Se ha salido del bucle principal.");
}
