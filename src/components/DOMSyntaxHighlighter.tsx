import { useMemo, useRef, useEffect, useState, useCallback, ReactElement } from 'react';

interface DOMSyntaxHighlighterProps {
  code: string;
  language: 'json';
  scrollTop: number;
  scrollLeft: number;
  fontSize: number;
  lineHeight: number;
  wordWrap: boolean;
  containerWidth?: number;
}

const PERFORMANCE_CONFIG = {
  VISIBLE_BUFFER_LINES: 20,
  LARGE_FILE_THRESHOLD: 500,
  CHUNK_SIZE: 100,
};

interface Token {
  type: 'string' | 'number' | 'boolean' | 'null' | 'key' | 'punctuation' | 'error' | 'text';
  value: string;
}

const tokenizeJSON = (code: string): Token[] => {
  const tokens: Token[] = [];
  let i = 0;

  const addToken = (type: Token['type'], value: string) => {
    tokens.push({ type, value });
  };

  while (i < code.length) {
    const char = code[i];

    if (char === '\n') {
      addToken('text', char);
      i++;
      continue;
    }

    if (/\s/.test(char)) {
      addToken('text', char);
      i++;
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

  return tokens;
};

export default function DOMSyntaxHighlighter({
  code,
  language,
  scrollTop,
  scrollLeft,
  fontSize,
  lineHeight,
  wordWrap,
  containerWidth
}: DOMSyntaxHighlighterProps) {
  const containerRef = useRef<HTMLPreElement>(null);
  const [visibleRange, setVisibleRange] = useState({ start: 0, end: 100 });
  const lineHeightPx = fontSize * lineHeight;
  
  const isDark = document.documentElement.getAttribute('data-theme') === 'dark';

  const lines = useMemo(() => code.split('\n'), [code]);
  const totalLines = lines.length;
  
  const isLargeFile = !wordWrap && totalLines > PERFORMANCE_CONFIG.LARGE_FILE_THRESHOLD;

  useEffect(() => {
    if (!isLargeFile || wordWrap) return;

    const containerHeight = containerRef.current?.parentElement?.clientHeight || 600;
    const visibleLines = Math.ceil(containerHeight / lineHeightPx);
    const startLine = Math.max(0, Math.floor(scrollTop / lineHeightPx) - PERFORMANCE_CONFIG.VISIBLE_BUFFER_LINES);
    const endLine = Math.min(totalLines, startLine + visibleLines + PERFORMANCE_CONFIG.VISIBLE_BUFFER_LINES * 2);

    setVisibleRange({ start: startLine, end: endLine });
  }, [scrollTop, lineHeightPx, totalLines, isLargeFile, wordWrap]);

  const renderLine = useCallback((lineContent: string, lineIndex: number) => {
    const tokens = tokenizeJSON(lineContent);
    
    return (
      <div key={lineIndex} className="syntax-line" data-line={lineIndex} style={{ minHeight: `${lineHeightPx}px` }}>
        {tokens.map((token, tokenIndex) => {
          if (token.value === '\n') return null;
          
          let className = '';
          switch (token.type) {
            case 'string': className = 'token-string'; break;
            case 'number': className = 'token-number'; break;
            case 'boolean': className = 'token-boolean'; break;
            case 'null': className = 'token-null'; break;
            case 'key': className = 'token-key'; break;
            case 'punctuation': className = 'token-punctuation'; break;
            case 'error': className = 'token-error'; break;
            default: className = 'token-text';
          }
          
          if (token.value === ' ') {
            return <span key={tokenIndex} className={className}>{'\u00A0'}</span>;
          }
          
          return <span key={tokenIndex} className={className}>{token.value}</span>;
        })}
      </div>
    );
  }, [lineHeightPx]);

  const highlightedContent = useMemo(() => {
    if (language !== 'json') return null;

    if (wordWrap) {
      return lines.map((line, index) => renderLine(line, index));
    }

    if (isLargeFile) {
      const elements: ReactElement[] = [];
      
      if (visibleRange.start > 0) {
        elements.push(
          <div
            key="top-spacer"
            style={{ height: `${visibleRange.start * lineHeightPx}px` }}
          />
        );
      }
      
      for (let i = visibleRange.start; i < visibleRange.end && i < totalLines; i++) {
        elements.push(renderLine(lines[i], i));
      }
      
      if (visibleRange.end < totalLines) {
        elements.push(
          <div
            key="bottom-spacer"
            style={{ height: `${(totalLines - visibleRange.end) * lineHeightPx}px` }}
          />
        );
      }
      
      return elements;
    } else {
      return lines.map((line, index) => renderLine(line, index));
    }
  }, [code, language, lines, totalLines, isLargeFile, visibleRange, lineHeightPx, renderLine, wordWrap]);

  if (language !== 'json' || !highlightedContent) {
    return null;
  }

  const contentWidthStyle = wordWrap && containerWidth
    ? { width: `${containerWidth - 32 - 10}px` }
    : {};
  const transformStyle = wordWrap
    ? { transform: `translateY(-${scrollTop}px)` }
    : { transform: `translate(-${scrollLeft}px, -${scrollTop}px)` };

  return (
    <pre
      ref={containerRef}
      className={`dom-syntax-highlighter ${isDark ? 'dark' : 'light'} ${wordWrap ? 'word-wrap-mode' : ''}`}
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: wordWrap ? 'auto' : 'max-content',
        minWidth: wordWrap ? 'auto' : '100%',
        height: 'auto',
        minHeight: '100%',
        margin: 0,
        padding: '16px',
        pointerEvents: 'none',
        zIndex: 1,
        fontFamily: "'Consolas', 'Monaco', 'Courier New', monospace",
        fontSize: `${fontSize}px`,
        lineHeight: lineHeight,
        whiteSpace: wordWrap ? 'pre-wrap' : 'pre',
        wordWrap: wordWrap ? 'break-word' : 'normal',
        overflowWrap: wordWrap ? 'break-word' : 'normal',
        wordBreak: wordWrap ? 'break-all' : 'normal',
        overflow: 'visible',
        boxSizing: 'border-box',
        letterSpacing: 'normal',
        wordSpacing: 'normal',
        tabSize: 2,
        ...contentWidthStyle,
        ...transformStyle,
      }}
    >
      {highlightedContent}
    </pre>
  );
}