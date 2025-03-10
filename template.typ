
 
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


                                                                          #set text(font: "Arial", size: 6pt)

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
                                
[5008000308353], [KARAOUI Nabil], [EXPRO HMD BASE 1], [Welder], [MISSING],
[500800027248], [ABBAD  Mansour], [WellTest], [Welltest Junior Field Operator 2], [MISSING],
[5008000308102], [AGOUNITESTANE Hamza], [Production], [Acting Technical Lead], [MISSING],
[5008000100279], [ABDENNOURI Tarek], [Supply Chain], [Logistics Coordinator 2], [MISSING],
[5008000308212], [MERZOUGI SAMIR], [EXPRO HMD BASE 2], [TRAINEE], [MISSING],
[500800024912], [AKERMA   Ghiles ], [WireLine], [Wireline Junior Field Technician 1], [MISSING],
[5008000308677], [ABD JABBAR Med Amine], [EXPRO HMD BASE 1], [Welder], [MISSING],
[5008000308389], [BEN SEDDIK Mohammed El Haddi], [Production], [Production Junior Field Technician 5], [MISSING],
[5008000238268], [AOUF  Rida], [WireLine], [Wireline Senior Field Technician 1], [MISSING],
[5008000308170], [ARIOUAT Toufik], [Production], [QA/QC ARH Specialist], [MISSING],
[5008000308354], [AMRI Anis], [EXPRO HMD BASE 1], [Welder], [MISSING],
[500800024884], [AICHI  Taha], [Production], [Production Junior Field Technician 4], [MISSING],
[5008000070471], [Khalfi Mourad ], [EXPRO HMD BASE 1], [Welder], [MISSING],
[500800025152], [AHMED SALAH  Abdesselam], [Meters ], [Meters Field Techncian 3], [MISSING],
[500800026497], [AKAB  Belkacem], [WellTest], [Welltest Junior Field Operator 3], [MISSING],
[500800026472], [ADAMOU  Ahmed Zakaria], [HSEQ], [QHSE Trainer Advisor 1], [MISSING],
[5008000213265], [AHMED SAID  Kahina], [HR], [HR Coordinator  1], [MISSING],
[500800010704], [ATTALAH  Messaoud], [WellTest], [ Welltest Field Specialist 1], [MISSING],
[500800024811], [ABDELAIDOUM  Abdelhakim], [Production], [MPP Senior Field Technician 3], [MISSING],
[5008000200991], [ABERKANE Hamid], [WellTest], [Welltest Junior Field Technician 1], [MISSING],
[5008000308676], [AMRANE Anouar], [EXPRO HMD BASE 1], [Clark Driver], [MISSING],
          )
        