
Main: ADA R1 , R2 , R3
      ADA R1 , R4 , R2

MAIN3:
    ADA R1, R2 , R4 ; comments Version2"
    LW R2, R1 ,  10
    LW R3, R4 , 0x0F
    ADI R1, R2, #10
    ADI R3, R2, 62
    SW R3 , RA , 100
    LLI R1, 0xFF
