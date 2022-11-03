import os
os.system("cargo build --release")
os.rename("./target/release/ae-plus-plus.exe", "./aepp.exe")