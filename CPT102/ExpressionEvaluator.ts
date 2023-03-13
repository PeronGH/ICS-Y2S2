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

  static isLeftParenthesis(token: Token | string): boolean {
    return token.toString() === "(";
  }

  isLeftParenthesis(): boolean {
    return Token.isLeftParenthesis(this);
  }

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
    const postfixTokens: Token[] = []; // Array to hold the postfix expression
    const operatorStack: Token[] = []; // Stack to hold operators

    for (const token of this._tokens) {
      if (token.isNumber()) { // If the token is a number, add it to the postfix expression
        postfixTokens.push(token);
        continue; // Move to the next token
      }

      if (token.isOperator()) { // If the token is an operator
        while (
          operatorStack.length > 0 && // While there are operators on the stack
          operatorStack[operatorStack.length - 1] // and the top operator on the stack
              .getPrecedence() >= token.getPrecedence() // has higher precedence than the current token
        ) {
          postfixTokens.push(operatorStack.pop()!); // Pop the top operator from the stack and add it to the postfix expression
        }
        operatorStack.push(token); // Push the current token onto the stack
        continue; // Move to the next token
      }

      if (token.isParenthesis()) { // If the token is a parenthesis
        if (token.isLeftParenthesis()) { // If the token is a left parenthesis, push it onto the stack
          operatorStack.push(token);
          continue; // Move to the next token
        }

        while (operatorStack.length > 0) { // If the token is a right parenthesis, pop operators from the stack
          const top = operatorStack.pop()!;
          if (top.isLeftParenthesis()) break; // until the matching left parenthesis is found
          postfixTokens.push(top); // Add the popped operators to the postfix expression
        }
      }
    }

    while (operatorStack.length > 0) { // Pop any remaining operators from the stack
      postfixTokens.push(operatorStack.pop()!); // and add them to the postfix expression
    }

    return postfixTokens; // Return the postfix expression
  }

  toPostfix(): string {
    return this.toPostfixTokens().join(" ");
  }
}

Deno.test("tokenize", () => {
  const tokens = Token.tokenize("1+2112*33/12");
  console.log(tokens);
});

Deno.test("to postfix", () => {
  const expr = new ExpressionEvaluator("1+2-5*3/4");
  console.log(expr.toPostfix());
});
