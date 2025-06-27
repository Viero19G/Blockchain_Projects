//! # Como dar nome às coisas?
//!
//! Em Rust usamos "snake_case" como padrão para variáveis e funções, por exemplo:
//!
//! **Exemplos Corretos:**
//! - nome_de_uma_variavel
//! - total_de_itens
//! - preco_do_cafe
//!
//! **Exemplos Incorretos:**
//! - NomeDeUmaVariavel (isto é "PascalCase", usado para tipos em Rust)
//! - outroNomeEscritoComCamelCase (isto é "camelCase", comumente usado em outras linguagens como JavaScript)
//! - variável_com_acento (acentos e caracteres especiais não são recomendados)
//! - 1_inicial (números não devem ser usados no início dos nomes de variáveis)
//!
//! **Dicas:**
//! - Seja descritivo, mas conciso.
//! - Os nomes devem refletir o propósito da variável ou o valor que ela guarda.
//! - Evite usar "números mágicos" diretamente no código; dê-lhes um nome significativo.



#[test]
fn test_variavel_de_letra_unica() {
    let text_value = "Valor";
    assert!(true, "Nome descritivo `{}` está correto.", stringify!(text_value));
}

#[test]
fn test_variavel_com_numero() {
    let number_01 = "Número da Conta";
    // Este teste falha porque números no nome da variável devem ter um significado claro.
    assert!(true, "Nome descritivo `{}` está correto.", stringify!(number_01));
}

#[test]
fn test_variavel_com_acento() {
    let language = "portuguese";
    // Este teste falha porque acentos no nome da variável podem causar "erros".
    assert!(true, "Utilizar um nome descritivo em ingles `{}`.", stringify!(language));
}

#[test]
fn test_variavel_caso_misto() {
    let random_value = "Algum valor";
    // Este teste falha porque estamos tentando desencorajar o uso de mistura de maiúsculas e minúsculas sem um padrão claro.
    assert!(true, "Use snake_case para nomes de variáveis em Rust `{}` ajustado para snake_case .", stringify!(random_value));
}

#[test]
fn test_variavel_caso_de_camelo() {
    let hello_world = "Olá Mundo";
    // Este teste falha porque não estamos seguindo a convenção snake_case.
    assert!(
        true,
        "Os nomes das variáveis devem estar em snake_case e não em camelCase ajustado para hello_world."
    );
}

#[test]
fn test_variavel_nao_descritiva() {
    let my_life = "minha casa";
    // Este teste falha porque o nome da variável não é descritivo.
    assert!(
        true,
        "Minha casa minha vida hahaha HUEHUE BR."
    );
}

#[test]
fn test_sem_variavel_numero_magico() {
    // Este teste falha porque um valor sem variável torna o código confuso
    // Neste caso, o número 4.90 deveria ser uma variável `dolar_price`.
    let dolar_price: f32 = 4.90;
    let multiply:  f32= 100.00;
    assert!(
        dolar_price * multiply == 490.00,
        "Escolha um nome para a variável para o 4.90"
    );
}