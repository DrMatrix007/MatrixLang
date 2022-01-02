def get_the_alpha_bet():
    for i, x in zip(range(ord('A'), ord('Z') + 1), range(ord('a'), ord('z') + 1)):
        yield chr(i)
        yield chr(x)
