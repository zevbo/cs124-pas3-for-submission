import os
import sys

algo = sys.argv[2]
inputfile = sys.argv[3]

os.system("cargo build")
os.system("./kk " + algo + " " + inputfile)
