#!/usr/bin/env python3

# -*- coding: utf-8 -*-

"""
    pi.py:
            Calculate Pi to the nth digit.
"""

__version__ = "0.1"

__author__ = "Christopher Brennan"
__copyright__ = "Copyright 2022, The Grunk Project"
__credits__ = ["Christopher Brennan" "w3resource.com"]
__license__ = "GPL"
__version__ = "0.1"
__maintainer__ = "Christopher Brennan"
__email__ = "chris@xaerolimit.net"
__status__ = "Alpha Testing"

import os, sys, math

#import <3rd party modules>

def CalculatePi(roundVal):

		somepi = round(math.pi,roundVal);
		pi = str(somepi)
		someList = list(pi)
		return somepi;
roundTo = input('How many places after Pi do you want to show?: ')
try:
	roundint = int(roundTo);
	print(CalculatePi(roundint));
except:
	print("You did not enter an integer");
