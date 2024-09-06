import os
import sys

import msvcrt

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
    showerror('XChat', 'XChat isn\'t installed.', parent=tkroot)
    sys.exit(2)
    
INSTCONF = XCHAT + r'\inst.conf'
print 'inst.conf: ' + INSTCONF
if os.path.isfile(INSTCONF):
    from win32api import *
    from win32con import *
    try:
        SetFileAttributes(INSTCONF, FILE_ATTRIBUTE_NORMAL)
        os.remove(INSTCONF)
        print 'deleted ' + INSTCONF
        print 'press any key'
        msvcrt.getch()
    except:
        print 'We need elevated privileges.'
        ShellExecute(None, 'runas', 'python', os.path.abspath(sys.argv[0]), None, 1)
        sys.exit(5)
