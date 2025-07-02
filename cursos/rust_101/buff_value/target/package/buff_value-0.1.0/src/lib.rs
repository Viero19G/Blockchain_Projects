/// A Rust library for calculating company valuation metrics based on Warren Buffett's principles.
// Biblioteca Rust para calcular métricas de valuation de empresas com base nos princípios de Warren Buffett.
pub mod wb_valuation{
    /// Calculates Owner's Earnings as per Buffett's definition.
    /// Formula: Net Income + Depreciation & Amortization - Maintenance CapEx
    // Calcula os Lucros do Proprietário conforme a definição de Buffett.
    // Fórmula: Lucro Líquido + Depreciação e Amortização - CapEx de Manutenção
    pub fn owners_earnings(net_income: f64, depreciation_amortization: f64, maintenance_capex: f64) -> f64 {
        net_income + depreciation_amortization - maintenance_capex
    }

    /// Calculates Return on Equity (ROE) as a percentage.
    /// Formula: (Net Income / Shareholders' Equity) * 100
    // Calcula o Retorno sobre o Patrimônio (ROE) como percentual.
    // Fórmula: (Lucro Líquido / Patrimônio dos Acionistas) * 100
    pub fn return_on_equity(net_income: f64, shareholders_equity: f64) -> Option<f64> {
        if shareholders_equity == 0.0 {
            None // Avoid division by zero
            // Evita divisão por zero
        } else {
            Some((net_income / shareholders_equity) * 100.0)
        }
    }

    /// Calculates Return on Net Tangible Assets (RONTA) as a percentage.
    /// Formula: (Net Income / (Total Assets - Total Liabilities - Intangible Assets)) * 100
    // Calcula o Retorno sobre Ativos Tangíveis Líquidos (RONTA) como percentual.
    // Fórmula: (Lucro Líquido / (Ativos Totais - Passivos Totais - Ativos Intangíveis)) * 100
    pub fn return_on_net_tangible_assets(net_income: f64, total_assets: f64, total_liabilities: f64, intangible_assets: f64) -> Option<f64> {
        let net_tangible_assets = total_assets - total_liabilities - intangible_assets;
        if net_tangible_assets == 0.0 {
            None // Avoid division by zero
            // Evita divisão por zero
        } else {
            Some((net_income / net_tangible_assets) * 100.0)
        }
    }

    /// Calculates Debt-to-Equity Ratio.
    /// Formula: Total Liabilities / Shareholders' Equity
    // Calcula a Razão Dívida/Patrimônio.
    // Fórmula: Passivos Totais / Patrimônio dos Acionistas
    pub fn debt_to_equity(total_liabilities: f64, shareholders_equity: f64) -> Option<f64> {
        if shareholders_equity == 0.0 {
            None // Avoid division by zero
            // Evita divisão por zero
        } else {
            Some(total_liabilities / shareholders_equity)
        }
    }

    /// Calculates Earnings Per Share (EPS).
    /// Formula: Net Income / Shares Outstanding
    // Calcula o Lucro por Ação (EPS).
    // Fórmula: Lucro Líquido / Ações em Circulação
    pub fn earnings_per_share(net_income: f64, shares_outstanding: f64) -> Option<f64> {
        if shares_outstanding == 0.0 {
            None // Avoid division by zero
            // Evita divisão por zero
        } else {
            Some(net_income / shares_outstanding)
        }
    }

    /// Calculates Compound Annual Growth Rate (CAGR) for EPS over a period.
    /// Formula: ((EPS_final / EPS_initial)^(1/years) - 1) * 100
    // Calcula a Taxa de Crescimento Anual Composta (CAGR) para o EPS em um período.
    // Fórmula: ((EPS_final / EPS_inicial)^(1/anos) - 1) * 100
    pub fn eps_cagr(initial_eps: f64, final_eps: f64, years: f64) -> Option<f64> {
        if initial_eps == 0.0 || years <= 0.0 {
            None // Avoid division by zero or invalid years
            // Evita divisão por zero ou anos inválidos
        } else {
            Some(((final_eps / initial_eps).powf(1.0 / years) - 1.0) * 100.0)
        }
    }

    /// Calculates intrinsic value using a simplified Discounted Cash Flow (DCF) model.
    /// Assumes constant owner's earnings growth for a period, then discounts to present value.
    /// Formula: Sum of (Owner's Earnings * (1 + growth_rate)^t / (1 + discount_rate)^t) for t=1 to years
    // Calcula o valor intrínseco usando um modelo simplificado de Fluxo de Caixa Descontado (DCF).
    // Assume crescimento constante dos lucros do proprietário por um período, depois desconta ao valor presente.
    // Fórmula: Soma de (Lucros do Proprietário * (1 + taxa_crescimento)^t / (1 + taxa_desconto)^t) para t=1 até anos
    pub fn intrinsic_value(
        initial_owners_earnings: f64,
        growth_rate: f64,
        discount_rate: f64,
        years: u32,
    ) -> f64 {
        let mut total_value = 0.0;
        for t in 1..=years {
            let t_f64 = t as f64;
            // Future owner's earnings: initial * (1 + growth_rate)^t
            // Lucros futuros do proprietário: inicial * (1 + taxa_crescimento)^t
            let future_earnings = initial_owners_earnings * (1.0 + growth_rate).powf(t_f64);
            // Present value: future_earnings / (1 + discount_rate)^t
            // Valor presente: lucros_futuros / (1 + taxa_desconto)^t
            let present_value = future_earnings / (1.0 + discount_rate).powf(t_f64);
            total_value += present_value;
        }
        total_value
    }

    /// Calculates intrinsic value per share.
    // Calcula o valor intrínseco por ação.
    pub fn intrinsic_value_per_share(
        initial_owners_earnings: f64,
        growth_rate: f64,
        discount_rate: f64,
        years: u32,
        shares_outstanding: f64,
    ) -> Option<f64> {
        if shares_outstanding == 0.0 {
            None // Avoid division by zero
            // Evita divisão por zero
        } else {
            Some(intrinsic_value(initial_owners_earnings, growth_rate, discount_rate, years) / shares_outstanding)
        }
    }
}
/// Unit tests for the valuation functions.
// Testes unitários para as funções de valuation.
#[cfg(test)]
mod tests {
    use super::wb_valuation::*;


    #[test]
    fn test_owners_earnings() {
        let result = owners_earnings(1000.0, 200.0, 150.0);
        assert_eq!(result, 1050.0); // 1000 + 200 - 150
        
    }

    #[test]
    fn test_return_on_equity() {
        let result = return_on_equity(500.0, 2000.0).unwrap();
        assert_eq!(result, 25.0); // (500 / 2000) * 100
        assert!(return_on_equity(500.0, 0.0).is_none());
        // Verifica que retorna None para patrimônio zero
    }

    #[test]
    fn test_return_on_net_tangible_assets() {
        let result = return_on_net_tangible_assets(500.0, 3000.0, 1000.0, 500.0).unwrap();
        assert_eq!(result, 33.33333333333333); // (500 / (3000 - 1000 - 500)) * 100
        assert!(return_on_net_tangible_assets(500.0, 1500.0, 1000.0, 500.0).is_none());
        // Verifica que retorna None para ativos tangíveis líquidos zero
    }

    #[test]
    fn test_debt_to_equity() {
        let result = debt_to_equity(1000.0, 2000.0).unwrap();
        assert_eq!(result, 0.5); // 1000 / 2000
        assert!(debt_to_equity(1000.0, 0.0).is_none());
        // Verifica que retorna None para patrimônio zero
    }

    #[test]
    fn test_earnings_per_share() {
        let result = earnings_per_share(1000.0, 100.0).unwrap();
        assert_eq!(result, 10.0); // 1000 / 100
        assert!(earnings_per_share(1000.0, 0.0).is_none());
        // Verifica que retorna None para ações em circulação zero
    }

    #[test]
    fn test_eps_cagr() {
        let result = eps_cagr(10.0, 14.641, 5.0).unwrap();
        assert!((result - 7.9).abs() < 0.1); // Approximate CAGR for (14.641/10)^(1/5) - 1
        // CAGR aproximado para (14.641/10)^(1/5) - 1
        assert!(eps_cagr(0.0, 14.641, 5.0).is_none());
        // Verifica que retorna None para EPS inicial zero
        assert!(eps_cagr(10.0, 14.641, 0.0).is_none());
        // Verifica que retorna None para anos inválidos
    }

    #[test]
    fn test_intrinsic_value() {
        let result = intrinsic_value(1000.0, 0.05, 0.1, 10);
        println!("o resultado é: {:?}", result);
        // Approximate expected value for 10 years of 1000 growing at 5%, discounted at 10%
        // Valor esperado aproximado para 10 anos de 1000 crescendo a 5%, descontado a 10%
        assert_eq!(result, 7811.80275662085); // Hand-calculated for verification
        // Calculado manualmente para verificação
    }

    #[test]
    fn test_intrinsic_value_per_share() {
        let result = intrinsic_value_per_share(1000.0, 0.05, 0.1, 10, 100.0).unwrap();
        // Divide intrinsic value by 100 shares
        // Divide o valor intrínseco por 100 ações
        assert!((result - 77.2173).abs() < 1.0); // Hand-calculated for verification
        // Calculado manualmente para verificação
        assert!(intrinsic_value_per_share(1000.0, 0.05, 0.1, 10, 0.0).is_none());
        // Verifica que retorna None para ações em circulação zero
    }
}