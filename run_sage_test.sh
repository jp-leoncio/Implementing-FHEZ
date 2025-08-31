# Este script executa o teste em Rust, captura os resultados, gera um script Sage,
# e compara as saídas para validação automática.

set -e # Encerra o script imediatamente se um comando falhar

echo "--- Passo 1: Executando teste em Rust para gerar saídas ---"
CARGO_OUTPUT=$(cargo test -q test_arithmetic_against_sage -- --nocapture)

# --- Extração dos Resultados do Rust ---
echo "--- Passo 2: Extraindo resultados do RUST ---"
RUST_SUM=$(echo "$CARGO_OUTPUT" | awk '/\[RUST_SUM\]/{flag=1; next} /\[RUST_MUL\]/{flag=0} flag')
RUST_MUL=$(echo "$CARGO_OUTPUT" | awk '/\[RUST_MUL\]/{flag=1; next} /-----------------------------------------/{flag=0} flag')

# --- Extração e Salvamento do Script Sage ---
echo "--- Passo 3: Extraindo e salvando o script SAGE ---"
SAGE_SCRIPT=$(echo "$CARGO_OUTPUT" | awk '/SAGE_SCRIPT_START/{flag=1; next} /SAGE_SCRIPT_END/{flag=0} flag')

if [ -z "$SAGE_SCRIPT" ]; then
    echo "ERRO: Não foi possível extrair o script Sage da saída do teste."
    exit 1
fi

# Garante que o diretório existe e salva o script em um arquivo
mkdir -p sage_files
echo "$SAGE_SCRIPT" > sage_files/generated_test.sage
echo "Script Sage salvo em 'sage_files/generated_test.sage'"

# --- Execução e Extração dos Resultados do Sage ---
echo "--- Passo 4: Executando script Sage e extraindo resultados ---"
# Executa o script que acabamos de salvar
SAGE_OUTPUT=$(sage sage_files/generated_test.sage)

SAGE_SUM_RAW=$(echo "$SAGE_OUTPUT" | awk '/\[SAGE_SUM\]/{flag=1; next} /\[SAGE_MUL\]/{flag=0} flag')
SAGE_MUL_RAW=$(echo "$SAGE_OUTPUT" | awk '/\[SAGE_MUL\]/{flag=1; next} /--- SAGE_RESULTS_END ---/{flag=0} flag') # O marcador final não existe, mas funciona para pegar até o fim

# Remove o prefixo "sage: " caso ele apareça
SAGE_SUM=$(echo "$SAGE_SUM_RAW" | sed 's/^sage: //')
SAGE_MUL=$(echo "$SAGE_MUL_RAW" | sed 's/^sage: //')

# --- Comparação dos Resultados ---
echo "--- Passo 5: Comparando os resultados de Rust e Sage ---"

# Remove espaços em branco para uma comparação robusta
RUST_SUM_NORMALIZED=$(echo "$RUST_SUM" | tr -d '[:space:]')
SAGE_SUM_NORMALIZED=$(echo "$SAGE_SUM" | tr -d '[:space:]')
RUST_MUL_NORMALIZED=$(echo "$RUST_MUL" | tr -d '[:space:]')
SAGE_MUL_NORMALIZED=$(echo "$SAGE_MUL" | tr -d '[:space:]')

TEST_PASSED=true

# Compara a SOMA
if [ "$RUST_SUM_NORMALIZED" == "$SAGE_SUM_NORMALIZED" ]; then
    echo "✅ SOMA: Os resultados são idênticos."
else
    echo "❌ SOMA: Os resultados são DIFERENTES!"
    echo "   RUST: $RUST_SUM"
    echo "   SAGE: $SAGE_SUM"
    TEST_PASSED=false
fi

# Compara a MULTIPLICAÇÃO
if [ "$RUST_MUL_NORMALIZED" == "$SAGE_MUL_NORMALIZED" ]; then
    echo "✅ MULTIPLICAÇÃO: Os resultados são idênticos."
else
    echo "❌ MULTIPLICAÇÃO: Os resultados são DIFERENTES!"
    echo "   RUST: $RUST_MUL"
    echo "   SAGE: $SAGE_MUL"
    TEST_PASSED=false
fi

# --- Resultado Final ---
echo "------------------------------------------------------------"
if [ "$TEST_PASSED" = true ]; then
    echo "🎉 SUCESSO: Todas as verificações passaram!"
    exit 0
else
    echo "🔥 FALHA: Uma ou mais verificações falharam."
    exit 1
fi