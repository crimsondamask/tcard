
 
                            #set page(paper: "a4", margin: (
                              top: 3cm,
                                bottom: 3cm,
                                  left: 2cm, 
                                right: 2cm,
                                              x: 1cm,
                                                  ), header: context {
                                                        [

                                                                _Expro Emergency Access Report_
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
                                                                                                    columns: (1fr, 1fr, 1fr, 1fr, 0.5fr),

                                                                                                      table.header[ID][Name][Department][Function][Status],
                                
[500800020584], [ABOTRABA Mahmoud], [WellTest], [Workshop / Maintenance Supervisor ], [MISSING],
[500800009716], [LAKRIB  Kamel], [WellTest], [ Welltest  Senior Field Technician 2], [MISSING],
[5008006000011], [Visiteur 11], [], [], [MISSING],
[500800011091993], [MADOUI Abdelkader], [DAQ  ], [DAQ Maintenance & PLC support Engineer], [MISSING],
[5008005000010], [Visiteur 10], [], [], [MISSING],
[500800024811], [ABDELAIDOUM  Abdelhakim], [Production], [MPP Senior Field Technician 3], [MISSING],
[500800016716], [LAKEHAL  Fares], [WellTest], [ Welltest Senior Field Technician 2], [MISSING],
[5008000308234], [ABBAS Karim], [Production], [Logistic and material coordinator], [MISSING],
          )
        Report date: 05-04-2025 17:37