
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Canteen Report_
                                                                    #h(1fr)
                                                                        #counter(page).display()
                                                                          ]
                                                                          }, )


                                                                          #set text(font: "Arial", size: 8pt)

                                                                          // Medium bold table header.
                                                                          #show table.cell.where(y: 0): set text(weight: "medium")

                                                                          // Bold titles.

                                                                          // See the strokes section for details on this!
                                                                          #let frame(stroke) = (x, y) => (
                                                                                left: if x > 0 { 0pt } else { stroke },
                                                                                  right: stroke,
                                                                                    top: if y < 2 { stroke } else { 0pt },
                                                                                      bottom: stroke,
                                                                                      )

                                                                                      #set table(
                                                                                            fill: (_, y) => if calc.odd(y) { rgb("EAF2F5") },
                                                                                              stroke: frame(rgb("21222C")),
                                                                                              )

                                                                                              #table(

                                                                                                    columns: (1fr, 1fr, 1fr, 1fr),
                                                                                                  table.header[Department][Breakfast][Lunch][Dinner],
                                [Area Support],[0],[0],[0],
[BD],[0],[0],[0],
[DAQ],[2],[1],[0],
[DST],[0],[0],[0],
[Facilities],[0],[0],[0],
[Finance],[0],[0],[0],
[Fluids],[0],[0],[0],
[HR],[0],[0],[0],
[QHSE],[1],[0],[0],
[IT],[1],[0],[0],
[L&D],[1],[0],[0],
[Wireline],[0],[0],[0],
[Meters],[0],[0],[0],
[Production],[0],[0],[0],
[OPS],[0],[0],[0],
[Supply Chain],[1],[0],[0],
[TRS],[1],[0],[0],
[Welltest],[5],[0],[0],
[Wireline],[0],[0],[0],
[Security],[0],[0],[0],

          )
        Report date: 12/06/2025

Breakfast total: 25

Lunch total: 1

Dinner total: 0

Total Meals: 26


            
          // Medium bold table header.
          #show table.cell.where(y: 0): set text(weight: "medium")

          // Bold titles.

          // See the strokes section for details on this!
          #let frame(stroke) = (x, y) => (
                left: if x > 0 { 0pt } else { stroke },
                  right: stroke,
                    top: if y < 2 { stroke } else { 0pt },
                      bottom: stroke,
                      )

                      #set table(
                            fill: (_, y) => if calc.odd(y) { rgb("EAF2F5") },
                              stroke: frame(rgb("21222C")),
                              )

                              #table(
                                        columns: (1fr, 1fr, 1fr, 0.5fr, 0.5fr, 0.5fr),

                                      table.header[ID][Name][Department][Breakfast][Lunch][Dinner],
        
[5008000308528], [BELLAKHDAR Abdeldjabar], [DAQ  ], [1], [0], [0],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [1], [1], [0],
[500800026472], [ADAMOU  Ahmed Zakaria], [HSEQ], [1], [0], [0],
[500800025380], [BENBEDJA  Salim], [IT], [1], [0], [0],
[500800019198], [BELKACEMI  Kahina], [L&D], [1], [0], [0],
[500800024811], [ABDELAIDOUM  Abdelhakim], [Production], [1], [0], [0],
[5008000308300], [BELFAR Imadeddine], [Production], [1], [0], [0],
[5008000308344], [BELFRIEKH Zakaria], [Production], [1], [0], [0],
[5008000308413], [BELHADJ  Mohamed Sadek], [Production], [1], [0], [0],
[5008000143333], [BELHAMRA   Yacine], [production], [1], [0], [0],
[5008000308313], [BELHANNACHI Hicham], [Production], [1], [0], [0],
[5008000308469], [BELKERCHA Houssem], [Production], [1], [0], [0],
[5008000070993], [Belkessour Younes], [Production], [1], [0], [0],
[5008000308381], [BELKHEIR  Hamza], [Production], [1], [0], [0],
[5008000308304], [BELMANI Oussama], [Production], [1], [0], [0],
[5008000308144], [BEN ABDALLAH Khaled], [Production], [1], [0], [0],
[5008000308389], [BEN SEDDIK Mohammed El Haddi], [Production], [1], [0], [0],
[5008000080791], [Zouari  Ferhat Hamza], [Production], [1], [0], [0],
[5008000308166], [BELHADI Imad  ], [Supply Chain], [1], [0], [0],
[500800020584], [ABOTRABA Mahmoud], [WellTest], [1], [0], [0],
[500800014374], [AGGOUN  Fayçal ], [WellTest], [1], [0], [0],
[50080000231194], [Bellouti Badreddine], [Welltest], [1], [0], [0],
[50080000130290], [Belouidiane Djaber], [Welltest], [1], [0], [0],
[5008000250659], [BENABDALLAH  Ahmed Abdelaziz], [WellTest], [1], [0], [0],
[5008000308498], [BELKADI  Saadi], [TRS / WC], [1], [0], [0],
          )
        