import utilities

import re

from typing import Dict, List

SPACES = " \t"

DIGITS = "0123456789."
VARIABLECHARS = [i for i in utilities.get_the_alpha_bet()] + ["_"]

INT_REG = "[0-9]+"
FLOAT_REG = "[0-9]*(\.){1}[0-9]+"
VARIABLE_REG = f"[{''.join(VARIABLECHARS)}]+"

INT_TOKEN = "m_int"
FLOAT_TOKEN = "m_float"
VARIABLE_TOKEN = "variable"

OPERATIONS_STR = {
    "+": 'PLUS',
    "-": 'MINUS',
    "*": 'MUL',
    "/": 'DIV',
    "(": 'LPAREN',
    ")": 'RPAREN',
    "=": 'ASSIGNMENT'
}


class Token:
    def __init__(self, token_type, value) -> None:
        self.type = token_type
        self.value = value

    def __repr__(self) -> str:
        if self.value:
            return f'{self.type}:{self.value}'
        return f'{self.type}'


class Lexer:
    def __init__(self, text) -> None:
        self.text = text
        self.pos = -1
        self.current_char = None
        self.advance()

    def advance(self):
        self.pos += 1
        self.current_char = self.text[self.pos] if self.pos < len(self.text) else None

    def make_number(self):
        val_str = ""
        while self.current_char is not None and self.current_char in DIGITS:
            val_str += self.current_char
            self.advance()
        self.pos -= 1
        if re.fullmatch(INT_REG, val_str):
            return Token(INT_TOKEN, int(val_str))
        if re.fullmatch(FLOAT_REG, val_str):
            return Token(FLOAT_TOKEN, float(val_str))

    def make_var(self):
        val_str = ""
        while self.current_char is not None and self.current_char in VARIABLECHARS:
            val_str += self.current_char
            self.advance()
        self.pos -= 1
        if re.fullmatch(VARIABLE_REG, val_str):
            return Token(VARIABLE_TOKEN, val_str)

    def make_tokens(self):
        tokens = []
        while self.current_char is not None:
            if self.current_char in SPACES:
                pass
            elif OPERATIONS_STR.keys().__contains__(self.current_char):
                tokens.append(OPERATIONS_STR[self.current_char])
            elif self.current_char in DIGITS:
                tokens.append(self.make_number())
            elif self.current_char in VARIABLECHARS:
                tokens.append(self.make_var())
            else:
                raise Exception(f"char '{self.current_char}' is not recognized")
            self.advance()

        return tokens


class Parser:
    def __init__(self, tokens: List[Token]):
        self.tokens = tokens
        self.pos = -1

    def get_current_token(self):
        return self.tokens[self.pos]

    def advance(self):
        self.pos += 1

    def analyze(self):
        while self.pos < len(self.tokens):



class Interpreter:
    def __init__(self):
        self.variables: Dict[str, object] = {}

    def analyze(self, data: str):
        lexer = Lexer(data)
        tokens = lexer.make_tokens()

        return tokens
