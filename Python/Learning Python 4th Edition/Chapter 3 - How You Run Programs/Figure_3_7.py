__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

a = 'dead'      # Define three attributes
b = 'parrot'    # Exported to other files
c = 'sketch'    # Tuples precursor

print(a, b, c)  # Also used in this file

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")