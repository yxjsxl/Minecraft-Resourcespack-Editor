interface Token {
  type: 'string' | 'number' | 'boolean' | 'null' | 'key' | 'punctuation' | 'error' | 'text';
  value: string;
  line: number;
  column: number;
}

interface TokenizeResult {
  tokens: Token[];
  lineCount: number;
  lineTokens: Map<number, Token[]>;
}

function tokenizeJSON(code: string): TokenizeResult {
  const tokens: Token[] = [];
  const lineTokens = new Map<number, Token[]>();
  let line = 0;
  let column = 0;
  let i = 0;

  const addToken = (type: Token['type'], value: string) => {
    const token = { type, value, line, column };
    tokens.push(token);
    
    if (!lineTokens.has(line)) {
      lineTokens.set(line, []);
    }
    lineTokens.get(line)!.push(token);
    
    column += value.length;
  };

  while (i < code.length) {
    const char = code[i];

    if (char === '\n') {
      addToken('text', char);
      line++;
      column = 0;
      i++;
      continue;
    }

    if (/\s/.test(char)) {
      let whitespace = '';
      while (i < code.length && /[ \t]/.test(code[i])) {
        whitespace += code[i];
        i++;
      }
      if (whitespace) {
        addToken('text', whitespace);
      }
      continue;
    }

    if (char === '"') {
      let str = '"';
      i++;
      let escaped = false;
      let closed = false;

      while (i < code.length) {
        const c = code[i];
        str += c;

        if (escaped) {
          escaped = false;
        } else if (c === '\\') {
          escaped = true;
        } else if (c === '"') {
          closed = true;
          i++;
          break;
        } else if (c === '\n') {
          break;
        }
        i++;
      }

      let j = i;
      while (j < code.length && /\s/.test(code[j])) j++;
      const isKey = code[j] === ':';

      addToken(closed ? (isKey ? 'key' : 'string') : 'error', str);
      continue;
    }

    if (/[0-9-]/.test(char)) {
      let num = '';
      while (i < code.length && /[0-9.eE+\-]/.test(code[i])) {
        num += code[i];
        i++;
      }
      const valid = /^-?(0|[1-9]\d*)(\.\d+)?([eE][+-]?\d+)?$/.test(num);
      addToken(valid ? 'number' : 'error', num);
      continue;
    }

    if (/[a-z]/.test(char)) {
      let word = '';
      while (i < code.length && /[a-z]/.test(code[i])) {
        word += code[i];
        i++;
      }

      if (word === 'true' || word === 'false') {
        addToken('boolean', word);
      } else if (word === 'null') {
        addToken('null', word);
      } else {
        addToken('error', word);
      }
      continue;
    }

    if ('{}[]:,'.includes(char)) {
      addToken('punctuation', char);
      i++;
      continue;
    }

    addToken('error', char);
    i++;
  }

  return {
    tokens,
    lineCount: line + 1,
    lineTokens
  };
}

function tokenizeIncremental(
  code: string,
  changedStartLine: number,
  changedEndLine: number,
  previousLineTokens: Map<number, Token[]>
): TokenizeResult {
  
  const lines = code.split('\n');
  const totalLines = lines.length;
  
  if (changedEndLine - changedStartLine < 100 && previousLineTokens.size > 0) {
    const newLineTokens = new Map<number, Token[]>();
    const allTokens: Token[] = [];
    
    for (let i = 0; i < changedStartLine && i < totalLines; i++) {
      const tokens = previousLineTokens.get(i);
      if (tokens) {
        newLineTokens.set(i, tokens);
        allTokens.push(...tokens);
      }
    }
    
    const changedCode = lines.slice(changedStartLine).join('\n');
    const changedResult = tokenizeJSON(changedCode);
    
    changedResult.lineTokens.forEach((tokens, lineIndex) => {
      const adjustedLine = lineIndex + changedStartLine;
      const adjustedTokens = tokens.map(t => ({
        ...t,
        line: adjustedLine
      }));
      newLineTokens.set(adjustedLine, adjustedTokens);
      allTokens.push(...adjustedTokens);
    });
    
    return {
      tokens: allTokens,
      lineCount: totalLines,
      lineTokens: newLineTokens
    };
  }
  
  return tokenizeJSON(code);
}

self.onmessage = (event: MessageEvent) => {
  const { id, type, data } = event.data;
  
  try {
    let result: any;
    
    switch (type) {
      case 'tokenize':
        result = tokenizeJSON(data.code);
        result.lineTokens = Array.from(result.lineTokens.entries());
        break;
        
      case 'tokenize-incremental':
        const previousLineTokens = new Map<number, Token[]>(data.previousLineTokens || []);
        result = tokenizeIncremental(
          data.code,
          data.changedStartLine,
          data.changedEndLine,
          previousLineTokens
        );
        result.lineTokens = Array.from(result.lineTokens.entries());
        break;
        
      default:
        throw new Error(`Unknown task type: ${type}`);
    }
    
    self.postMessage({ id, type, result });
  } catch (error) {
    self.postMessage({ 
      id, 
      type, 
      error: error instanceof Error ? error.message : 'Unknown error' 
    });
  }
};