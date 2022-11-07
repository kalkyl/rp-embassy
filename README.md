# rp-embassy

# **********\*\*********** Start of STEP Patterns ******************\*\*\*******************

# f_fs1_32bit and b_fs1_32bit

# • FULL STEP one Phase coils ON/OFF pattern

FULL_STEP = [1, 2, 4, 8] # Dubled to have 32 bit

# forward step pattern 1 2 4 8 1 2 4 8

f_fs1_32bit = 306713160 #0001 0010 0100 1000 0001 0010 0100 1000 32 bit

# backwards step pattern 8 4 2 1 8 4 2 1

b_fs1_32bit = 2216789025 #1000 0100 0010 0001 1000 0100 0010 0001 32 bit

# • FULL STEP Two Phases

FULL_STEP_2 = [3, 6, 2, 9] # Dubled to have 32 bit

# forward step pattern 3 6 12 9 3 6 12 9

f_fs2_32bit = 919156425 #0011 0110 1100 1001 0011 0110 1100 1001 32 bit

# backwards step pattern 9 12 6 3 9 12 6 3

b_fs2_32bit = 2623773795 #1001 1100 0110 0011 1001 1100 0110 0011 32 bit

# • HALF STEP (mixed One & Two Phases)

HALF_STEP = [1, 3, 2, 6, 4, 12, 8, 9]

# forward step pattern 1 3 2 6 4 12 8 9

f_hs = 321277065 #0001 0011 0010 0110 0100 1100 1000 1001 32 bit

# backwards step pattern 9 8 12 4 6 2 3 1

b_hs = 2563007025 #1001 1000 1100 0100 0110 0010 0011 0001 32 bit

# **********\*\*********** End of STEP Patterns ******************\*\*\*******************
