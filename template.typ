
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Canteen Report Ikram Base_
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
[DAQ],[1],[1],[1],
[DST],[0],[0],[0],
[Facilities],[0],[0],[0],
[Finance],[0],[0],[0],
[Fluids],[0],[0],[0],
[HR],[0],[0],[0],
[QHSE],[0],[0],[0],
[IT],[0],[0],[0],
[L&D],[0],[0],[0],
[Meters],[0],[0],[0],
[Production],[0],[0],[0],
[OPS],[0],[0],[0],
[Supply Chain],[0],[0],[0],
[TRS],[0],[0],[0],
[Welltest],[1],[0],[0],
[Wireline],[0],[0],[0],
[Security],[0],[0],[0],
[Visitors],[0],[0],[0],

          )
        Report start date: 2025/09/05 00:00:00 +0000

Report end date: 2025/09/06 00:00:00 +0000

Breakfast total: 2

Lunch total: 1

Dinner total: 1

Total Meals: 4


            
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
        
[500800025140], [ABDOU  Abdelhalim], [Sampling], [Dinner],
[500800025140], [ABDOU  Abdelhalim], [Sampling], [Lunch],
[500800025140], [ABDOU  Abdelhalim], [Sampling], [Breakfast],
[50080000030193], [Abid Sifeddine], [Welltest], [Breakfast],
          )
        