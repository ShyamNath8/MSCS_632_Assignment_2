# Dynamic typing: variable can hold different types
def adder(x, y):
    return x + y  # '+' is polymorphic: numeric add or string concat

print(adder(2, 3))         # 5 (int)
print(adder(2.5, 1.5))     # 4.0 (float)
print(adder("hi ", "you")) # "hi you" (str)

# Scope example
def outer():
    a = 10
    def inner():
        nonlocal a
        a += 5
        return a
    return inner()

print("outer->inner:", outer())  # demonstrates closure + nonlocal
