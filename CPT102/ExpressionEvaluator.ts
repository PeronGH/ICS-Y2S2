class Token {
  constructor(private _name: string) {}

  toString(): string {
    return this._name;
  }

  private static _OPERATOR_PRECEDENCE = new Map(
    [
      ["+", 1],
      ["-", 1],
      ["*", 2],
      ["/", 2],
    ],
  );

  static isParenthesis(token: Token | string): boolean {
    const parenthesis = ["(", ")"];
    return parenthesis.includes(token.toString());
  }

  isParenthesis(): boolean {
    return Token.isParenthesis(this);
  }

  static isOperator(token: Token | string): boolean {
    return this._OPERATOR_PRECEDENCE.has(token.toString());
  }

  isOperator(): boolean {
    return Token.isOperator(this);
  }

  static isSymbol(token: Token | string): boolean {
    return Token.isParenthesis(token) || Token.isOperator(token);
  }

  isSymbol(): boolean {
    return Token.isSymbol(this);
  }

  static isNumber(token: Token | string): boolean {
    const number = Number(token.toString());
    return number >= 0;
  }

  isNumber(): boolean {
    return Token.isNumber(this);
  }

  static isWhitespace(token: Token | string): boolean {
    return token.toString().trim() === "";
  }

  isWhitespace(): boolean {
    return Token.isWhitespace(this);
  }

  static getPrecedence(token: Token | string): number {
    return Token._OPERATOR_PRECEDENCE.get(token.toString()) ?? 0;
  }

  getPrecedence(): number {
    return Token.getPrecedence(this);
  }

  static tokenize(expression: string): Token[] {
    const tokens: Token[] = [];
    let currentToken = "";

    for (const char of expression) {
      if (Token.isWhitespace(char)) continue;

      if (Token.isSymbol(char)) {
        if (currentToken) {
          tokens.push(new Token(currentToken));
          currentToken = "";
        }
        tokens.push(new Token(char));
        continue;
      }

      currentToken += char;
    }

    if (currentToken) {
      tokens.push(new Token(currentToken));
    }

    return tokens;
  }
}

class ExpressionEvaluator {
  private _tokens: Token[];

  /**
   * @param tokens an infix expression
   */
  constructor(tokens: string | Token[]) {
    if (typeof tokens === "string") {
      this._tokens = Token.tokenize(tokens);
    } else {
      this._tokens = tokens;
    }
  }

  toPostfixTokens(): Token[] {
    const postfixTokens: Token[] = [];
    const operatorStack: Token[] = [];

    for (const token of this._tokens) {
      if (token.isNumber()) {
        postfixTokens.push(token);
        continue;
      }

      if (token.isOperator()) {
        while (
          operatorStack.length > 0 &&
          operatorStack[operatorStack.length - 1].getPrecedence() >=
            token.getPrecedence()
        ) {
          postfixTokens.push(operatorStack.pop()!);
        }
        operatorStack.push(token);
        continue;
      }

      if (token.isParenthesis()) {
        if (token.toString() === "(") {
          operatorStack.push(token);
          continue;
        }

        while (operatorStack.length > 0) {
          const top = operatorStack.pop()!;
          if (top.toString() === "(") break;
          postfixTokens.push(top);
        }
      }
    }

    while (operatorStack.length > 0) {
      postfixTokens.push(operatorStack.pop()!);
    }

    return postfixTokens;
  }
}

Deno.test("tokenize", () => {
  const tokens = Token.tokenize("1+2112*33/12");
  console.log(tokens);
});

Deno.test("to postfix", () => {
  const expr = new ExpressionEvaluator("1+2*3/4");
  console.log(expr.toPostfixTokens().join(" "));
});
