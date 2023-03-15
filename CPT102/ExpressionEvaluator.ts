enum TokenType {
  NUMBER,
  OPERATOR,
  PARENTHESIS,
  LETTER,
}

type OperatorData = {
  precedence: number;
};

class Token {
  constructor(public readonly type: TokenType, public readonly value: string) {}

  toString(): string {
    return this.value;
  }

  private static OPERATOR_DATA = new Map<string, OperatorData>(
    [
      ["+", { precedence: 1 }],
      ["-", { precedence: 1 }],
      ["*", { precedence: 2 }],
      ["/", { precedence: 2 }],
    ],
  );

  static isParenthesis(token: Token | string): boolean {
    const parenthesis = ["(", ")"];
    return parenthesis.includes(token.toString());
  }

  get isParenthesis(): boolean {
    return Token.isParenthesis(this);
  }

  static isOperator(token: Token | string): boolean {
    return this.OPERATOR_DATA.has(token.toString());
  }

  get isOperator(): boolean {
    return Token.isOperator(this);
  }

  static isSymbol(token: Token | string): boolean {
    return Token.isParenthesis(token) || Token.isOperator(token);
  }

  get isSymbol(): boolean {
    return Token.isSymbol(this);
  }

  static isNumber(token: Token | string): boolean {
    return /^\d+$/.test(token.toString());
  }

  get isNumber(): boolean {
    return Token.isNumber(this);
  }

  static isWhitespace(token: Token | string): boolean {
    return token.toString().trim() === "";
  }

  get isWhitespace(): boolean {
    return Token.isWhitespace(this);
  }

  static getPrecedence(token: Token | string): number {
    return Token.OPERATOR_DATA.get(token.toString())?.precedence ?? 0;
  }

  get precedence(): number {
    return Token.getPrecedence(this);
  }

  static tokenize(expression: string): Token[] {
    // Step 1: Initialize an empty array to hold the tokens
    const tokens: Token[] = [];

    // Step 2: Initialize a buffer to hold the characters of the current token
    let buffer = "";

    // Step 3: Iterate through each character of the expression
    for (let i = 0; i < expression.length; i++) {
      const char = expression[i];
      const nextChar = expression[i + 1];

      // Step 4: Check if the current character is a symbol
      if (Token.isSymbol(char)) {
        // Step 5: If the buffer is not empty, push a number token to the tokens array
        if (buffer) {
          tokens.push(new Token(TokenType.NUMBER, buffer));
          buffer = "";
        }

        // Step 6: Handle negative numbers with prefix "-"
        if (
          char === "-" && Token.isNumber(nextChar) &&
          (!tokens.length ||
            Token.isOperator(tokens[tokens.length - 1].value) ||
            tokens[tokens.length - 1].value === "(")
        ) {
          buffer += "-";
        } else {
          // Step 7: Push an operator token to the tokens array
          tokens.push(new Token(TokenType.OPERATOR, char));
        }
      } else if (Token.isNumber(char)) {
        // Step 8: If the current character is a number, add it to the buffer
        buffer += char;
      } else if (Token.isWhitespace(char)) {
        // Step 9: If the current character is whitespace and the buffer is not empty,
        // push a number token to the tokens array
        if (buffer) {
          tokens.push(new Token(TokenType.NUMBER, buffer));
          buffer = "";
        }
      } else {
        // Step 10: If the current character is not a symbol, number, or whitespace, throw an error
        throw new Error(`Unexpected token: ${char}`);
      }
    }

    // Step 11: If the buffer is not empty, push a number token to the tokens array
    if (buffer) {
      tokens.push(new Token(TokenType.NUMBER, buffer));
    }

    // Step 12: Return the tokens array
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
    return ExpressionEvaluator.toPostfixTokens(this._tokens);
  }

  toPostfix(): string {
    return this.toPostfixTokens().join(" ");
  }

  private static toPostfixTokens(tokens: Token[]): Token[] {
    // Step 1: Initialize an empty array to hold the output tokens
    const output: Token[] = [];

    // Step 2: Initialize an empty stack to hold the operators
    const stack: Token[] = [];

    // Step 3: Define a function to handle operators
    const handleOperator = (operator: Token) => {
      while (
        stack.length &&
        stack[stack.length - 1].type === TokenType.OPERATOR &&
        (
          operator.precedence <= stack[stack.length - 1].precedence ||
          operator.precedence < stack[stack.length - 1].precedence
        )
      ) {
        output.push(stack.pop()!);
      }
      stack.push(operator);
    };

    // Step 4: Iterate through each token in the input
    for (const token of tokens) {
      if (token.type === TokenType.NUMBER) {
        // Step 5: If the token is a number, push it to the output
        output.push(token);
      } else if (token.type === TokenType.OPERATOR) {
        // Step 6: If the token is an operator, handle it
        handleOperator(token);
      } else if (token.value === "(") {
        // Step 7: If the token is a left parenthesis, push it to the stack
        stack.push(token);
      } else if (token.value === ")") {
        // Step 8: If the token is a right parenthesis, pop operators from the stack and push them to the output until a left parenthesis is encountered
        while (stack.length && stack[stack.length - 1].value !== "(") {
          output.push(stack.pop()!);
        }
        if (!stack.length) {
          throw new Error("Mismatched parentheses");
        }
        stack.pop();
      }
    }

    // Step 9: Pop any remaining operators from the stack and push them to the output
    while (stack.length) {
      if (
        stack[stack.length - 1].value === "(" ||
        stack[stack.length - 1].value === ")"
      ) {
        throw new Error("Mismatched parentheses");
      }
      output.push(stack.pop()!);
    }

    // Step 10: Return the output array
    return output;
  }
}

Deno.test("tokenize", () => {
  const tokens = Token.tokenize("(-1+2112)* -33/12");
  console.log(tokens);
});

Deno.test("to postfix", () => {
  const expr = new ExpressionEvaluator("1+2-5*3/4");
  console.log(expr.toPostfix());
});
