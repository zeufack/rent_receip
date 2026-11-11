use std::collections::BTreeMap;

use chrono::NaiveDate;

/// Everything that varies from one receipt to the next.
#[derive(Debug, Clone)]
pub struct Receipt {
    pub number: String,
    pub date: String,
    pub period: String,
    pub period_start: String,
    pub period_end: String,
    pub lessor_name: String,
    pub lessor_address: String,
    pub lessee_name: String,
    pub lessee_address: String,
    pub lessee_civility: String,
    pub property_address: String,
    pub city: String,
    pub rent: u64,
    pub charges: u64,
    pub energy: u64,
    pub total: u64,
    pub currency: String,
    pub payment_method: String,
    pub payment_date: String,
}

impl Receipt {
    /// Flat string map handed to the Typst template.
    pub fn fields(&self) -> BTreeMap<&'static str, String> {
        let mut fields = BTreeMap::new();
        fields.insert("number", self.number.clone());
        fields.insert("date", self.date.clone());
        fields.insert("period", self.period.clone());
        fields.insert("period_start", self.period_start.clone());
        fields.insert("period_end", self.period_end.clone());
        fields.insert("lessor_name", self.lessor_name.clone());
        fields.insert("lessor_address", self.lessor_address.clone());
        fields.insert("lessee_name", self.lessee_name.clone());
        fields.insert("lessee_address", self.lessee_address.clone());
        fields.insert(
            "lessee_display",
            format!("{} {}", self.lessee_civility, self.lessee_name)
                .trim()
                .to_string(),
        );
        fields.insert("property_address", self.property_address.clone());
        fields.insert("city", self.city.clone());
        fields.insert("rent", group_digits(self.rent));
        fields.insert("charges", group_digits(self.charges));
        fields.insert("energy", group_digits(self.energy));
        fields.insert("total", group_digits(self.total));
        fields.insert("total_words", amount_in_words(self.total, &self.currency));
        fields.insert("currency", currency_display(&self.currency, self.total));
        fields.insert("payment_method", self.payment_method.clone());
        fields.insert("payment_date", self.payment_date.clone());
        fields
    }

    pub fn default_file_name(&self) -> String {
        let period = self
            .period
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .to_lowercase();
        let lessee = self
            .lessee_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .to_lowercase();
        format!("quittance-{period}-{lessee}.pdf")
    }
}

pub fn group_digits(number: u64) -> String {
    let digits = number.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

/// Accepts `YYYY-MM` and turns it into `Novembre 2026`.
/// Any other input is passed through unchanged.
pub fn normalize_period(raw: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Janvier",
        "Février",
        "Mars",
        "Avril",
        "Mai",
        "Juin",
        "Juillet",
        "Août",
        "Septembre",
        "Octobre",
        "Novembre",
        "Décembre",
    ];

    let mut parts = raw.split('-');
    let (Some(year), Some(month)) = (parts.next(), parts.next()) else {
        return raw.to_string();
    };
    if parts.next().is_some() {
        return raw.to_string();
    }
    let (Ok(year), Ok(month)) = (year.parse::<u32>(), month.parse::<usize>()) else {
        return raw.to_string();
    };
    match month.checked_sub(1).and_then(|index| MONTHS.get(index)) {
        Some(name) => format!("{name} {year}"),
        None => raw.to_string(),
    }
}

/// For a `YYYY-MM` period, returns the first and last day as `dd/mm/yyyy`.
pub fn period_bounds(raw: &str) -> Option<(String, String)> {
    let mut parts = raw.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) {
        return None;
    }

    let start = NaiveDate::from_ymd_opt(year, month, 1)?;
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let end = NaiveDate::from_ymd_opt(next_year, next_month, 1)?.pred_opt()?;

    Some((
        start.format("%d/%m/%Y").to_string(),
        end.format("%d/%m/%Y").to_string(),
    ))
}

pub fn amount_in_words(amount: u64, currency: &str) -> String {
    format!(
        "{} {}",
        number_to_words_fr(amount),
        currency_words(currency, amount > 1)
    )
}

fn currency_display(currency: &str, amount: u64) -> String {
    match currency.to_lowercase().as_str() {
        "euro" | "euros" => if amount > 1 { "euros" } else { "euro" }.to_string(),
        "dollar" | "dollars" => if amount > 1 { "dollars" } else { "dollar" }.to_string(),
        "livre" | "livres" => if amount > 1 { "livres" } else { "livre" }.to_string(),
        _ => currency.to_string(),
    }
}

fn currency_words(currency: &str, plural: bool) -> String {
    let (singular, plural_form) = match currency.to_uppercase().as_str() {
        "EUR" | "EURO" | "EUROS" | "€" => ("euro", "euros"),
        "FCFA" | "XAF" | "XOF" => ("franc CFA", "francs CFA"),
        "USD" | "$" => ("dollar", "dollars"),
        "GBP" | "£" => ("livre sterling", "livres sterling"),
        _ => return currency.to_lowercase(),
    };
    if plural { plural_form } else { singular }.to_string()
}

const UNITS: [&str; 20] = [
    "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix", "onze",
    "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf",
];

const TENS: [&str; 7] = [
    "",
    "",
    "vingt",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
];

fn below_100(n: u32) -> String {
    if n < 20 {
        return UNITS[n as usize].to_string();
    }
    match n / 10 {
        2..=6 => {
            let base = TENS[(n / 10) as usize];
            match n % 10 {
                0 => base.to_string(),
                1 => format!("{base} et un"),
                unit => format!("{base}-{}", UNITS[unit as usize]),
            }
        }
        7 => {
            if n == 71 {
                "soixante et onze".to_string()
            } else {
                format!("soixante-{}", UNITS[(n - 60) as usize])
            }
        }
        8 => {
            if n == 80 {
                "quatre-vingts".to_string()
            } else {
                format!("quatre-vingt-{}", UNITS[(n - 80) as usize])
            }
        }
        9 => format!("quatre-vingt-{}", UNITS[(n - 80) as usize]),
        _ => unreachable!(),
    }
}

fn below_1000(n: u32) -> String {
    let hundreds = n / 100;
    let remainder = n % 100;
    if hundreds == 0 {
        return below_100(remainder);
    }
    let mut out = if hundreds == 1 {
        "cent".to_string()
    } else {
        format!("{} cent", UNITS[hundreds as usize])
    };
    if remainder == 0 {
        if hundreds > 1 {
            out.push('s');
        }
    } else {
        out.push(' ');
        out.push_str(&below_100(remainder));
    }
    out
}

/// `cent`/`vingt` lose their plural `s` when they multiply `mille`.
fn strip_plural_before_mille(value: &str) -> String {
    if let Some(prefix) = value.strip_suffix("cents") {
        format!("{prefix}cent")
    } else if let Some(prefix) = value.strip_suffix("vingts") {
        format!("{prefix}vingt")
    } else {
        value.to_string()
    }
}

pub fn number_to_words_fr(n: u64) -> String {
    match n {
        0 => "zéro".to_string(),
        1..=999 => below_1000(n as u32),
        1_000..=999_999 => {
            let thousands = n / 1_000;
            let remainder = n % 1_000;
            let mut out = if thousands == 1 {
                "mille".to_string()
            } else {
                format!(
                    "{} mille",
                    strip_plural_before_mille(&below_1000(thousands as u32))
                )
            };
            if remainder > 0 {
                out.push(' ');
                out.push_str(&below_1000(remainder as u32));
            }
            out
        }
        1_000_000..=999_999_999 => {
            let millions = n / 1_000_000;
            let remainder = n % 1_000_000;
            let mut out = if millions == 1 {
                "un million".to_string()
            } else {
                format!("{} millions", below_1000(millions as u32))
            };
            if remainder > 0 {
                out.push(' ');
                out.push_str(&number_to_words_fr(remainder));
            }
            out
        }
        _ => {
            let billions = n / 1_000_000_000;
            let remainder = n % 1_000_000_000;
            let mut out = if billions == 1 {
                "un milliard".to_string()
            } else {
                format!("{} milliards", below_1000(billions as u32))
            };
            if remainder > 0 {
                out.push(' ');
                out.push_str(&number_to_words_fr(remainder));
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_amounts_in_french() {
        assert_eq!(number_to_words_fr(0), "zéro");
        assert_eq!(number_to_words_fr(1), "un");
        assert_eq!(number_to_words_fr(17), "dix-sept");
        assert_eq!(number_to_words_fr(21), "vingt et un");
        assert_eq!(number_to_words_fr(71), "soixante et onze");
        assert_eq!(number_to_words_fr(80), "quatre-vingts");
        assert_eq!(number_to_words_fr(81), "quatre-vingt-un");
        assert_eq!(number_to_words_fr(90), "quatre-vingt-dix");
        assert_eq!(number_to_words_fr(99), "quatre-vingt-dix-neuf");
        assert_eq!(number_to_words_fr(100), "cent");
        assert_eq!(number_to_words_fr(200), "deux cents");
        assert_eq!(number_to_words_fr(201), "deux cent un");
        assert_eq!(number_to_words_fr(1_000), "mille");
        assert_eq!(number_to_words_fr(200_000), "deux cent mille");
        assert_eq!(number_to_words_fr(350_000), "trois cent cinquante mille");
        assert_eq!(number_to_words_fr(1_000_000), "un million");
    }

    #[test]
    fn normalizes_periods() {
        assert_eq!(normalize_period("2026-11"), "Novembre 2026");
        assert_eq!(normalize_period("Novembre 2026"), "Novembre 2026");
    }

    #[test]
    fn computes_period_bounds() {
        assert_eq!(
            period_bounds("2026-11"),
            Some(("01/11/2026".to_string(), "30/11/2026".to_string()))
        );
        assert_eq!(
            period_bounds("2024-02"),
            Some(("01/02/2024".to_string(), "29/02/2024".to_string()))
        );
    }

    #[test]
    fn groups_digits() {
        assert_eq!(group_digits(350_000), "350 000");
        assert_eq!(group_digits(1_000_000), "1 000 000");
    }

    #[test]
    fn writes_amounts_with_correct_currency_plural() {
        assert_eq!(amount_in_words(1, "EUR"), "un euro");
        assert_eq!(amount_in_words(0, "euro"), "zéro euro");
        assert_eq!(amount_in_words(550, "euro"), "cinq cent cinquante euros");
        assert_eq!(
            amount_in_words(350_000, "FCFA"),
            "trois cent cinquante mille francs CFA"
        );
    }
}
