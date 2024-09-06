__author__ = 'Christopher Brennan'
__email__ = 'xaero@xaerolimit.net'
__version__ = '0.1'

print("Question #1")
print("Q: How can you start an interactive interpreter session?")
print("A: There are several ways to open the interactive interpreter, the easiest way is to drop to a command prompt")
print("   in Windows or to open a terminal in a Unix/Linux/OSX environment and type 'python' (without the quotes).")
print()

print("Question #2")
print("Q: Where do you type a system command line to launch a script file?")
print("A: In a terminal window (Unix/Linux/OSX) or in a command (cmd) prompt (In Microsoft Windows)")
print()

print("Question #3")
print("Q:  Name four or more ways to run the code saved in a script file.")
print("A1: python script.py")
print("A2: ./script.py")
print("A3: double click script.py from a GUI env.")
print("A4: From inside an IDE")
print()

print("Question #4")
print("Q: What [are] two pitfalls related to clicking file icons on Windows?")
print("A1: The system command prompt will open and close before you can see all of the output of the program.")
print("A2: If there are errors, it can't be seen to begin the debugging process.")
print()

print("Question #5")
print("Q: What might you need to reload a module?")
print("A: 'import' or 'from' statements")
print()

print("Question #6")
print("Q: How do you run a script from within IDLE?")
print("A: Select 'Run -> Run Module' menu option ")
print()

print("Question #7")
print("Q: Name two pitfalls [are] related to using IDLE.")
print("A1: Users must add '.py' file extension")
print("A2: Users of IDLE need not reimport already imported modules from previous scripts while in the same session.")
print()

print("Question #8")
print("Q: What is a namespace, and how does it relate to module files?")
print("A: A namespace is the space where Python stores all of it's names, it's unique to each module and names"
      "of the same name can exist in the same top-level calling script.")
print()

# Stop console from exiting.
end_prog = ""
while end_prog != "q":
    end_prog = input("Hit 'q' to quit: ")