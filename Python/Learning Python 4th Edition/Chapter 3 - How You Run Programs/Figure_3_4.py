__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

# Module imports
import sys
import os

#Variable assignments
x = os.getcwd()
xy = sys.platform
xyz = 'Spam!'
a = 2 ** 100

#Print statements
print('Current working directory: ',x)
print('Your platform is: ',xy)
print('Spam to the power of 8 is: ',xyz * 8)
print('2 to the power of 100 is: ', a)

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")
