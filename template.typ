
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Canteen Report Main Base_
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
[DAQ],[4],[2],[1],
[DST],[0],[0],[0],
[Facilities],[0],[0],[0],
[Finance],[0],[0],[0],
[Fluids],[0],[0],[0],
[HR],[0],[0],[0],
[QHSE],[1],[1],[0],
[IT],[2],[0],[0],
[L&D],[1],[0],[0],
[Meters],[0],[0],[0],
[Production],[13],[2],[0],
[OPS],[0],[0],[0],
[Supply Chain],[1],[0],[0],
[TRS],[1],[0],[0],
[Welltest],[5],[2],[0],
[Wireline],[0],[0],[0],
[Security],[2],[0],[0],
[Visitors],[0],[0],[0],

          )
        Report start date: 2025/03/06 00:00:00 +0000

Report end date: 2025/08/07 00:00:00 +0000

Breakfast total: 32

Lunch total: 7

Dinner total: 1

Total Meals: 40


            
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

                                      table.header[ID][Name][Department][Meal],
        
[5008000308664], [SERHANI Imad Eddine], [EXPRO HMD BASE 1], [Breakfast],
[5008000308665], [YAHIACHERIF  Messaoud], [EXPRO HMD BASE 1], [Breakfast],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Breakfast],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Dinner],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Lunch],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Breakfast],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Breakfast],
[5008000070791], [Ouabdelkader Nadir], [DAQ  ], [Lunch],
[5008000308528], [BELLAKHDAR Abdeldjabar], [DAQ  ], [Breakfast],
[500800026472], [ADAMOU  Ahmed Zakaria], [HSEQ], [Breakfast],
[500800026472], [ADAMOU  Ahmed Zakaria], [HSEQ], [Lunch],
[500800025380], [BENBEDJA  Salim], [IT], [Breakfast],
[500800025380], [BENBEDJA  Salim], [IT], [Breakfast],
[500800019198], [BELKACEMI  Kahina], [L&D], [Breakfast],
[5008000080791], [Zouari  Ferhat Hamza], [Production], [Breakfast],
[500800024811], [ABDELAIDOUM  Abdelhakim], [Production], [Breakfast],
[5008000308300], [BELFAR Imadeddine], [Production], [Breakfast],
[5008000308344], [BELFRIEKH Zakaria], [Production], [Breakfast],
[5008000308413], [BELHADJ  Mohamed Sadek], [Production], [Breakfast],
[5008000143333], [BELHAMRA   Yacine], [production], [Breakfast],
[5008000308313], [BELHANNACHI Hicham], [Production], [Breakfast],
[5008000308469], [BELKERCHA Houssem], [Production], [Breakfast],
[5008000070993], [Belkessour Younes], [Production], [Breakfast],
[5008000308381], [BELKHEIR  Hamza], [Production], [Breakfast],
[5008000308304], [BELMANI Oussama], [Production], [Breakfast],
[5008000308144], [BEN ABDALLAH Khaled], [Production], [Breakfast],
[5008000308389], [BEN SEDDIK Mohammed El Haddi], [Production], [Breakfast],
[500800024811], [ABDELAIDOUM  Abdelhakim], [Production], [Lunch],
[5008000308127], [AGGOUN Nour Elislam], [Production], [Lunch],
[5008000308166], [BELHADI Imad  ], [Supply Chain], [Breakfast],
[500800020584], [ABOTRABA Mahmoud], [WellTest], [Breakfast],
[500800014374], [AGGOUN  Fayçal ], [WellTest], [Breakfast],
[50080000231194], [Bellouti Badreddine], [Welltest], [Breakfast],
[50080000130290], [Belouidiane Djaber], [Welltest], [Breakfast],
[5008000250659], [BENABDALLAH  Ahmed Abdelaziz], [WellTest], [Breakfast],
[500800020584], [ABOTRABA Mahmoud], [WellTest], [Lunch],
[500800014374], [AGGOUN  Fayçal ], [WellTest], [Lunch],
[5008000308498], [BELKADI  Saadi], [TRS / WC], [Breakfast],
          )
        