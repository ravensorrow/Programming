__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

import Figure_3_6_mod1                      # Import a whole python script as a module
import Figure_3_6_mod2                      # Import a whole python script as a module
from Figure_3_6_mod1 import title1          # Importing a specific func/var from a script/module
from Figure_3_6_mod2 import title2          # Importing a specific func/var from a script/module

print("Title 1: ", Figure_3_6_mod1.title)   # Call the specific func/var from a specific script/module
print("Title 2: ", Figure_3_6_mod2.title)   # Call the specific func/var from a specific script/module
print("Title 1: ", Figure_3_6_mod1.title)   # Call the specific func/var from a specific script/module

print("Title 1: ", title2)                  # Call the function/var as if it were native
print("Title 2: ", title1)                  # Call the function/var as if it were native

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")