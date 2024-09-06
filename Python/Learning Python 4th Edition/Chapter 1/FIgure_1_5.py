__author__ = 'Chris'
__email__ = 'xaero@xaerolimit.net'

print("Firstish Grade")
print('1 + 1 =', 1+1)          # Lets add some numbers!
print('2 + 4 =', 2+4)          # Yup, still adding numbers
print('5 - 2 =', 5-2)          # Still addi... oh nope, we're subtracting now
print()                         # Awkward silence?
print("Thirdish Grade")
print("243 - 23 =", 243-23)    # Subtraction of bigger numbers
print("12 * 4 =", 12*4)        # What is this!?! Oh right, it's multiplication!
print("12 / 3 =", 12 / 3)       # Division Bell?
print("13 / 3 =", 13 // 3, "R", 13 % 3)     # Division with a remainder
print()                         # More awkward silence
print("Junior High")
print("123.56 - 62.12 =", 123.56 - 62.12)   # Decimal subtraction
print("(4 + 3) * 2 =", (4 + 3) * 2)         # PEMDAS math
print("4 + 3 * 2 =", 4 + 3 * 2)             # PEMDAS math
print("3 ** 2 =", 3 ** 2)                   # Powers
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")
