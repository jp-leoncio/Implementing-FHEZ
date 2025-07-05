#[allow(non_snake_case)]
pub fn somar(a: f64, b: f64) -> f64 {
    a + b
}

pub fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::process::Command;
    use std::str;
    use regex::Regex;

    /// Executa o script Sage e captura sua saída
    fn sage_script(script_path: &str) -> Result<String, std::io::Error> {
        let output = Command::new("sage")
            .arg(script_path)
            .output()?;

        if output.status.success() {
            Ok(str::from_utf8(&output.stdout).unwrap().to_string())
        } else {
            let error_message = format!(
                "Sage script failed with error: {}",
                str::from_utf8(&output.stderr).unwrap_or("Could not parse stderr")
            );
            Err(std::io::Error::new(std::io::ErrorKind::Other, error_message))
        }
    }

    /// Analisa a saída do Sage
    fn parse_sage(output: &str) -> HashMap<String, String> {
        let mut results = HashMap::new();
        let regex = Regex::new(r"(?m)^([^:]+):(.*)$").unwrap();
        for cap in regex.captures_iter(output) {
            results.insert(cap[1].to_string(), cap[2].trim().to_string());
        }
        results
    }

    fn compare(val1: f64, val2: f64) -> bool {
        const EPSILON: f64 = 1e-9;
        (val1 - val2).abs() < EPSILON
    }

    #[test]
    fn comparacao_sage() {
        // 1. Executar o script Sage e analisar os resultados
        let sage_output = match sage_script("sage_files/example.sage") {
            Ok(output) => output,
            Err(e) => panic!("A execução do script Sage falhou. Erro retornado: {}", e),
        };
        let sage_results = parse_sage(&sage_output);

        // 2. Definir valores e executar a lógica em Rust
        let a = 1.234;
        let b = 5.678;
        let soma_rust = somar(a, b);
        let produto_rust = multiplicar(a, b);

        // 3. Obter e comparar o resultado da soma
        let soma_sage_str = sage_results.get("soma").expect("Chave 'soma' não encontrada na saída do Sage.");
        let soma_sage = soma_sage_str.parse::<f64>().expect("Falha ao converter a soma do Sage para f64.");
        
        assert!(
            compare(soma_rust, soma_sage),
            "Resultado da soma não corresponde! Rust: {}, Sage: {}",
            soma_rust,
            soma_sage
        );

        // 4. Obter e comparar o resultado do produto
        let produto_sage_str = sage_results.get("produto").expect("Chave 'produto' não encontrada na saída do Sage.");
        let produto_sage = produto_sage_str.parse::<f64>().expect("Falha ao converter o produto do Sage para f64.");

        assert!(
            compare(produto_rust, produto_sage),
            "Resultado do produto não corresponde! Rust: {}, Sage: {}",
            produto_rust,
            produto_sage
        );
    }

    #[test]
    fn test_comparacao_simbolica_com_sage() {
        // 1. Executar e analisar
        let sage_output = sage_script("sage_files/example.sage")
            .expect("Falha ao executar o script Sage.");
        let sage_results = parse_sage(&sage_output);

        // 2. Obter e comparar o resultado simbólico
        let expressao_sage = sage_results.get("expressao expandida").expect("Chave 'expressao expandida' não encontrada.");
        
        // A representação em Rust pode ser uma string pré-definida
        let expressao_rust_esperada = "x^2 + 2*x + 1";

        // A comparação de strings simbólicas pode ser sensível a espaços
        // Normalizamos ambas as strings removendo os espaços
        assert_eq!(
            expressao_sage.replace(" ", ""),
            expressao_rust_esperada.replace(" ", ""),
            "A expansão simbólica não corresponde!"
        );
    }
}