import sys

from platform import architecture

from _winreg import OpenKey, QueryValueEx, KEY_QUERY_VALUE
from _winreg import HKEY_LOCAL_MACHINE as HKLM, HKEY_CURRENT_USER as HKCU

ARCH, x = architecture()
if ARCH == '64bit':
    key = r'SOFTWARE\Wow6432Node\xchat'
else:
    key = r'SOFTWARE\xchat'
    
try:
    with OpenKey(HKLM, key, 0, KEY_QUERY_VALUE) as key:
        XCHAT, x = QueryValueEx(key, 'Install_Dir')
except WindowsError:
    from Tkinter import Tk
    from tkMessageBox import showerror
    tkroot = Tk()
    tkroot.withdraw()
    showerror('XChat', 'XChat is not installed', parent=tkroot)
    sys.exit(2)
    
print XCHAT

