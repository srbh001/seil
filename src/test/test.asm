
Main: AWC R1 , R2 , R3
      ADA R1 , R2 , R

MAIN3:
    ADA R1, R2 , R4 ; comments Version2"
    LW R2, R1 ,  10
    LW R3, R4 , 0xFF
    ADI R1, R2, #1000
    ADI R3, R2, 100
    SW R3 , R2 , 100
