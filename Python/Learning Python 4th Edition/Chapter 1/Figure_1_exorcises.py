__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

MyName = 'Christopher Brennan'
MyDOB = '09/08/1979'

print('My name is:', MyName)
print('My date of birth is:', MyDOB)

print('=-=-=-=-=-=-')
print('1 + 1 =', 1+1)
print('1 - 1 =', 1-1)
print('1 * 1 =', 1*1)
print('9 / 3 =', 9/3)
print('(5 * 2) * 10 + ( 8 / 2 ) ** 9 =', (5*2)*10+(8/2)**9)

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")
