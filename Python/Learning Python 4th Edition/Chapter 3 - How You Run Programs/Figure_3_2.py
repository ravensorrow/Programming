__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

spam = 'Spam|' * 8
print(spam)

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")