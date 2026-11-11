use rent_receip::receipt::Receipt;
use rent_receip::render::{Renderer, TypstRenderer};

fn sample_receipt() -> Receipt {
    Receipt {
        number: "Q-0001".to_string(),
        date: "08/11/2026".to_string(),
        period: "Novembre 2026".to_string(),
        period_start: "05/11/2026".to_string(),
        period_end: "04/12/2026".to_string(),
        lessor_name: "Jean Dupont".to_string(),
        lessor_address: "12 rue des Palmiers".to_string(),
        lessee_name: "Marie Kamga".to_string(),
        lessee_address: "45 boulevard de la Liberté".to_string(),
        lessee_civility: "Madame".to_string(),
        property_address: "Appartement 3, immeuble Les Palmiers".to_string(),
        city: "Douala".to_string(),
        rent: 350_000,
        charges: 25_000,
        energy: 0,
        total: 375_000,
        currency: "FCFA".to_string(),
        payment_method: "Espèces".to_string(),
        payment_date: "05/11/2026".to_string(),
    }
}

#[test]
fn renders_a_valid_pdf() {
    let pdf = TypstRenderer::new()
        .render(&sample_receipt())
        .expect("the receipt should render");

    assert!(pdf.starts_with(b"%PDF-"), "output should be a PDF");
    assert!(pdf.len() > 1_000, "output should not be empty");
}
