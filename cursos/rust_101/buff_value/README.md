# 📈 buff_value

**A Rust library for calculating company valuation metrics based on Warren Buffett's investment principles.**  
**Uma biblioteca Rust para calcular métricas de valuation de empresas com base nos princípios de investimento de Warren Buffett.**

---

## 🧠 What is it? / O que é?

`buff_value` is a Rust library that provides tools for value investing analysis inspired by Warren Buffett. It helps investors and developers estimate intrinsic company value using fundamental metrics like Owner's Earnings, ROE, RONTA, EPS, and DCF.

`buff_value` é uma biblioteca Rust que oferece ferramentas para análise de investimento em valor inspiradas em Warren Buffett. Ela auxilia investidores e desenvolvedores a estimar o valor intrínseco de uma empresa com base em métricas fundamentais como Lucros do Proprietário, ROE, RONTA, EPS e Fluxo de Caixa Descontado.

---

## 🧩 Module: `wb_valuation`

This module includes the following functions:  
Este módulo inclui as seguintes funções:

| Function                            | Description (EN)                                                       | Descrição (PT-BR)                                                  |
|-------------------------------------|------------------------------------------------------------------------|---------------------------------------------------------------------|
| `owners_earnings`                  | Calculates Owner's Earnings                                            | Calcula os Lucros do Proprietário                                  |
| `return_on_equity`                | Return on Equity (ROE)                                                 | Retorno sobre o Patrimônio (ROE)                                   |
| `return_on_net_tangible_assets`   | Return on Net Tangible Assets (RONTA)                                  | Retorno sobre Ativos Tangíveis Líquidos (RONTA)                    |
| `debt_to_equity`                  | Debt-to-Equity Ratio                                                   | Razão Dívida/Patrimônio                                            |
| `earnings_per_share`             | Earnings Per Share (EPS)                                               | Lucro por Ação (EPS)                                               |
| `eps_cagr`                        | EPS Compound Annual Growth Rate (CAGR)                                 | CAGR do EPS (Taxa de Crescimento Anual Composta)                   |
| `intrinsic_value`                | Intrinsic Value via Discounted Cash Flow (DCF)                         | Valor Intrínseco via Fluxo de Caixa Descontado (DCF)               |
| `intrinsic_value_per_share`      | Intrinsic Value per Share                                              | Valor Intrínseco por Ação                                          |

---

## 🛠 Installation / Instalação

Add to your `Cargo.toml`:  
Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
buff_value = { path = "caminho/para/buff_value" }
