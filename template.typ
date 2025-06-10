
 
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
                                                                                                    columns: (1fr, 1fr, 1fr, 0.5fr, 0.5fr, 0.5fr),

                                                                                                      table.header[ID][Name][Department][Breakfast][Lunch][Dinner],
                                
[500800011091993], [MADOUI Abdelkader], [DAQ  ], [1], [1], [1],
          )
        
            
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
[Wireline],[0],[0],[0],
[Meters],[0],[0],[0],
[Production],[0],[0],[0],
[OPS],[0],[0],[0],
[Supply Chain],[0],[0],[0],
[TRS],[0],[0],[0],
[Welltest],[0],[0],[0],
[Wireline],[0],[0],[0],

          )
        Report date: 10-06-2025 20:48

Breakfast total: 1

Lunch total: 1

Dinner total: 1

Dinner total: 1

