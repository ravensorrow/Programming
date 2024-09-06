__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

a = 123.4
b23 = 'Spam'
first_name = 'Bill'
b = 432
c = a + b
print('a + b is', c)
print('first_name is', first_name)
print('Sorted Parts, After Midnight or', b23)

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")