use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAPCODES: Vec<(&'static str, &'static str, &'static str)> = vec![
        ("-3887", "Total Revenue", "Income Statement"),
        ("-984", "EBITDA", "Income Statement"),
        ("-4524", "Operating Income", "Income Statement"),
        ("-3994", "Net Income", "Income Statement"),
        ("-5370", "Revenue Per Share", "Income Statement"),
        ("-3851", "EPS  Net Income - Basic", "Income Statement"),
        ("-4023", "EPS  Net Income - Diluted", "Income Statement"),
        ("-4043", "Shares Outstanding", "Income Statement"),
        ("-4021", "Weighted Average Shares Outstanding - Basic", "Income Statement"),
        ("-4022", "Weighted Average Shares Outstanding - Diluted", "Income Statement"),
        ("-4041", "Current Assets", "Balance Sheet"),
        ("-873", "Total Assets", "Balance Sheet"),
        ("-4042", "Current Liabilities", "Balance Sheet"),
        ("-8396", "Long Term Debt", "Balance Sheet"),
        ("-965", "Total Liabilities", "Balance Sheet"),
        ("-4497", "Stockholders' Equity", "Balance Sheet"),
        ("-9087", "Total Assets Per Share", "Balance Sheet"),
        ("-10424", "Net Assets Per Share", "Balance Sheet"),
        ("-976", "Cash From Operations", "Cash Flow"),
        ("-4341", "Cash From Investing", "Cash Flow"),
        ("-4340", "Cash From Financing", "Cash Flow"),
        ("-5377", "Cash, Beginning of Year", "Cash Flow"),
        ("-5378", "Cash, End of Year", "Cash Flow"),
        ("-7562", "Cash Flow Per Share", "Cash Flow"),
        ("-3842", "Revenue Per Employee: Annualized Revenue / Number of Employees", "Derived"),
        ("-1464", "Net Income Per Employee: Annualized Net Income / Number of Employees", "Derived"),
        ("-5009", "Return on Assets: (Net Income / Average Total Assets Over Period) * 100", "Derived"),
        ("-5011", "Return on Equity: (Net Income / Average Stockholders' Equity Over Period) * 100", "Derived"),
        ("-6153", "Return on Investment: (Operating Income / Average Invested Capital Over Period) * 100", "Derived"),
        ("-4067", "Gross Margin: (Gross Profit / Revenue) * 100", "Derived"),
        ("-5917", "Operating Margin: (Operating Income / Revenue) * 100", "Derived"),
        ("-1012", "EBITDA Margin: (EBITDA / Revenue) * 100", "Derived"),
        ("-4074", "Net Margin: (Net Income / Revenue) * 100", "Derived"),
        ("-1368", "Calculated Tax Rate: (Income Taxes / Earnings Before Taxes) * 100", "Derived"),
        ("-1766", "Total Asset Turnover: Annualized Revenue / Average Total Assets", "Derived"),
        ("-1791", "Receivables Turnover: Annualized Revenue / Average Receivables - net", "Derived"),
        ("-1793", "Inventory Turnover: Annualized Direct Exp excl deprec / Average Inventories or Annualized Revenue / Average Inventories if Direct Expenses are not available", "Derived"),
        ("-1771", "PP&E Turnover: Annualized Revenue / Average PPE - net", "Derived"),
        ("-6047", "Cash & Equivalents Turnover: Annualized Revenue / Average Cash & Equivs", "Derived"),
        ("-1800", "Accounts Payable Turnover: Annualized Revenue / Average Accounts Payable", "Derived"),
        ("-1802", "Accrued Expenses Turnover: Annualized Revenue / Average Accrued Expenses", "Derived"),
        ("-1016", "Interest Coverage: Operating Income / (0 - Non-Operating Net Interest Income)", "Derived"),
        ("-1017", "Long Term Debt to Equity: Long-Term Obligations / Shareholders' Equity",	"Derived"),
        ("-6159", "Total Debt to Equity: Total Debt & Leases / Shareholders' Equity",	"Derived"),
        ("-959", "Quick Ratio: Quick Assets / Current Liabilities", "Derived"),
        ("-966", "Net Current Assets as % of Total Assets: (Net Current Assets / Total Assets) * 100", "Derived"),
        ("-1465", "Free Cash Flow Per Share: Free Cash Flow (Annualized) / Weighted Average Shares Outstanding (Basic)", "Derived"),
        ("-9085", "Revenue to Assets: Revenue / Total Assets", "Derived"),
        ("-4051", "Accounts Payable", "Balance Sheet"),
        ("-4050", "Receivables", "Balance Sheet"),
        ("-4049", "Cash and Equivalents", "Balance Sheet"),
        ("-4019", "Direct Expenses", "Income Statement"),
        ("-4008", "Inventories", "Balance Sheet")
    ];
}