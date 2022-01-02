import MLang

interpreter = MLang.Interpreter()

while True:
    text = input("MLang > ")
    print(interpreter.analyze(text))
