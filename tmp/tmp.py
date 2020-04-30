f = open("symact.txt")
for l in f.read().split("\n"):
    if "WINDOW:Talk(SymAct(" not in l:
        if "WINDOW:DrawFace" not in l:
            print(l)
