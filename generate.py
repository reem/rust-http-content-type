import os, subprocess, urllib2

# Download and write the mimes to a file
content = urllib2.urlopen('http://svn.apache.org/repos/asf/httpd/httpd/trunk/docs/conf/mime.types').read().decode()
open('src/generator/mimes.txt', 'w').write(content)

# Compile and run the generator
try:
    os.makedirs('target/generator')
except:
    pass # In case the directory already exists
subprocess.call('rustc -O -o target/generator/generator src/generator/main.rs')
subprocess.call('target/generator/generator src/generator/mimes.txt src/mimes.rs')