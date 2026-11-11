// Rent receipt (quittance de loyer) template.
// Layout follows the "Location vide ou meublée à titre de résidence principale"
// model. Values are injected from Rust via `sys.inputs`.
#let field(key) = sys.inputs.at(key, default: "")

#set page(paper: "a4", margin: (x: 1.7cm, top: 1.4cm, bottom: 1.2cm))
#set text(font: "Libertinus Serif", size: 9.5pt, lang: "fr")
#set par(justify: true, leading: 0.65em)

// Title band
#block(
  width: 100%,
  fill: rgb("#cfe3ef"),
  inset: (x: 10pt, y: 5pt),
  align(center)[
    #text(size: 10pt)[Location vide ou meublée à titre de résidence principale :] \
    #text(size: 11pt, weight: "bold")[Quittance de loyer]
  ]
)

#v(0.35cm)

// Main bordered frame
#block(
  width: 100%,
  stroke: 0.7pt + black,
  inset: (x: 16pt, y: 14pt),
)[
  #align(right)[#text(size: 8pt, fill: luma(110))[N° #field("number")]]

  #v(0.15cm)

  #align(center)[
    #text(size: 18pt, weight: "bold")[Quittance de loyer du mois de #field("period")]
  ]

  #v(1.2cm)

  #grid(
    columns: (1fr, 1fr),
    align: (left, right),
    [
      #emph[#field("lessor_name")] \
      #emph[#field("lessor_address")]
    ],
    [
      #emph[#field("lessee_display")] \
      #emph[#field("lessee_address")]
    ],
  )

  #v(0.5cm)
  #align(right)[Fait à #field("city"), le #field("date")]

  #v(0.9cm)
  #text(weight: "bold")[Adresse de la location :] \
  #field("property_address")

  #v(1.8cm)

  Je soussigné #strong(field("lessor_name")) propriétaire du logement désigné ci-dessus, déclare
  avoir reçu de #strong(field("lessee_display")), la somme de #strong[#field("total_words")] /
  #field("total") #field("currency"), au titre du paiement du loyer et des charges pour la période
  de location du #field("period_start") au #field("period_end") et lui en donne quittance, sous
  réserve de tous mes droits.

  #v(1cm)

  #text(weight: "bold")[Détail du règlement :]
  #v(0.2cm)

  Loyer : #field("rent") #field("currency")
  #v(0.3cm)
  Provision pour charges : #field("charges") #field("currency")
  #v(0.3cm)
  (le cas échéant, contribution aux économies d'énergies) : #field("energy") #field("currency")
  #v(0.3cm)
  #text(weight: "bold")[Total :] #field("total") #field("currency")

  #v(0.9cm)

  Date du paiement : le #field("payment_date")

  #if field("payment_method") != "" [
    #v(0.2cm)
    Mode de paiement : #field("payment_method")
  ]

  #v(4.8cm)

  #text(size: 8pt, style: "italic")[
    (En bas de page) Cette quittance annule tous les reçus qui auraient pu être établis
    précédemment en cas de paiement partiel du montant du présent terme. Elle est à conserver
    pendant trois ans par le locataire (loi n° 89-462 du 6 juillet 1989 : art. 7-1).
  ]
]
