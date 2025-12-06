import { useEffect, useRef, useMemo, useCallback, useState } from 'react';

interface HighPerformanceCanvasHighlighterProps {
  code: string;
  language: 'json';
  scrollTop: number;
  scrollLeft: number;
  fontSize: number;
  lineHeight: number;
}

interface Token {
  type: 'string' | 'number' | 'boolean' | 'null' | 'key' | 'punctuation' | 'error' | 'text';
  value: string;
  line: number;
  column: number;
}

const PERFORMANCE_CONFIG = {
  CHUNK_SIZE: 50,
  BUFFER_CHUNKS: 3,
  MAX_CACHED_CHUNKS: 30,
  SCROLL_THROTTLE_MS: 8,
  PRELOAD_DELAY_MS: 50,
  LARGE_FILE_THRESHOLD: 5000,
  WORKER_THRESHOLD: 1000,
};

const TOKEN_COLORS_DARK: Record<string, string> = {
  string: '#ce9178',
  number: '#b5cea8',
  boolean: '#569cd6',
  null: '#569cd6',
  key: '#9cdcfe',
  punctuation: '#d4d4d4',
  error: '#f48771',
  text: '#d4d4d4',
};

const TOKEN_COLORS_LIGHT: Record<string, string> = {
  string: '#a31515',
  number: '#098658',
  boolean: '#0000ff',
  null: '#0000ff',
  key: '#0451a5',
  punctuation: '#333333',
  error: '#cd3131',
  text: '#333333',
};

class CharWidthCache {
  private cache: Map<string, number> = new Map();
  private ctx: CanvasRenderingContext2D | null = null;
  private currentFont: string = '';

  setContext(ctx: CanvasRenderingContext2D, font: string) {
    if (this.currentFont !== font) {
      this.cache.clear();
      this.currentFont = font;
    }
    this.ctx = ctx;
  }

  getWidth(char: string): number {
    if (!this.ctx) return 0;
    
    let width = this.cache.get(char);
    if (width === undefined) {
      width = this.ctx.measureText(char).width;
      this.cache.set(char, width);
    }
    return width;
  }

  getStringWidth(str: string): number {
    let width = 0;
    for (const char of str) {
      width += this.getWidth(char);
    }
    return width;
  }

  clear() {
    this.cache.clear();
  }
}

class ChunkCacheManager {
  private chunkCanvases: Map<number, OffscreenCanvas | HTMLCanvasElement> = new Map();
  private chunkVersions: Map<number, string> = new Map();
  private maxCachedChunks: number;
  private accessOrder: number[] = [];
  private totalMemoryEstimate: number = 0;
  private maxMemoryBytes: number = 200 * 1024 * 1024;

  constructor(maxChunks: number = PERFORMANCE_CONFIG.MAX_CACHED_CHUNKS) {
    this.maxCachedChunks = maxChunks;
  }

  getChunk(chunkIndex: number): OffscreenCanvas | HTMLCanvasElement | undefined {
    const canvas = this.chunkCanvases.get(chunkIndex);
    if (canvas) {
      const idx = this.accessOrder.indexOf(chunkIndex);
      if (idx > -1) {
        this.accessOrder.splice(idx, 1);
      }
      this.accessOrder.push(chunkIndex);
    }
    return canvas;
  }

  setChunk(
    chunkIndex: number, 
    canvas: OffscreenCanvas | HTMLCanvasElement, 
    version: string
  ) {
    const memoryUsage = canvas.width * canvas.height * 4;
    
    while (
      (this.chunkCanvases.size >= this.maxCachedChunks || 
       this.totalMemoryEstimate + memoryUsage > this.maxMemoryBytes) &&
      this.accessOrder.length > 0
    ) {
      this.evictOldest();
    }

    const existingCanvas = this.chunkCanvases.get(chunkIndex);
    if (existingCanvas) {
      this.totalMemoryEstimate -= existingCanvas.width * existingCanvas.height * 4;
    }

    this.chunkCanvases.set(chunkIndex, canvas);
    this.chunkVersions.set(chunkIndex, version);
    this.totalMemoryEstimate += memoryUsage;
    
    const idx = this.accessOrder.indexOf(chunkIndex);
    if (idx > -1) {
      this.accessOrder.splice(idx, 1);
    }
    this.accessOrder.push(chunkIndex);
  }

  getVersion(chunkIndex: number): string | undefined {
    return this.chunkVersions.get(chunkIndex);
  }

  private evictOldest() {
    if (this.accessOrder.length > 0) {
      const oldest = this.accessOrder.shift()!;
      const canvas = this.chunkCanvases.get(oldest);
      if (canvas) {
        this.totalMemoryEstimate -= canvas.width * canvas.height * 4;
      }
      this.chunkCanvases.delete(oldest);
      this.chunkVersions.delete(oldest);
    }
  }

  invalidateChunk(chunkIndex: number) {
    const canvas = this.chunkCanvases.get(chunkIndex);
    if (canvas) {
      this.totalMemoryEstimate -= canvas.width * canvas.height * 4;
    }
    this.chunkCanvases.delete(chunkIndex);
    this.chunkVersions.delete(chunkIndex);
    const idx = this.accessOrder.indexOf(chunkIndex);
    if (idx > -1) {
      this.accessOrder.splice(idx, 1);
    }
  }

  clear() {
    this.chunkCanvases.clear();
    this.chunkVersions.clear();
    this.accessOrder = [];
    this.totalMemoryEstimate = 0;
  }

  getStats() {
    return {
      cachedChunks: this.chunkCanvases.size,
      maxChunks: this.maxCachedChunks,
      memoryUsageMB: (this.totalMemoryEstimate / (1024 * 1024)).toFixed(2),
      maxMemoryMB: (this.maxMemoryBytes / (1024 * 1024)).toFixed(0)
    };
  }
}

class ScrollStateManager {
  private lastScrollTop: number = 0;
  private lastScrollLeft: number = 0;
  private scrollVelocityY: number = 0;
  private lastScrollTime: number = 0;
  private isScrolling: boolean = false;
  private scrollEndTimeout: number | null = null;

  update(scrollTop: number, scrollLeft: number): {
    deltaY: number;
    deltaX: number;
    velocityY: number;
    isScrolling: boolean;
    direction: 'up' | 'down' | 'none';
  } {
    const now = performance.now();
    const timeDelta = now - this.lastScrollTime;
    
    const deltaY = scrollTop - this.lastScrollTop;
    const deltaX = scrollLeft - this.lastScrollLeft;
    
    if (timeDelta > 0) {
      this.scrollVelocityY = deltaY / timeDelta * 1000;
    }
    
    this.lastScrollTop = scrollTop;
    this.lastScrollLeft = scrollLeft;
    this.lastScrollTime = now;
    this.isScrolling = true;
    
    if (this.scrollEndTimeout) {
      clearTimeout(this.scrollEndTimeout);
    }
    
    this.scrollEndTimeout = window.setTimeout(() => {
      this.isScrolling = false;
      this.scrollVelocityY = 0;
    }, 150);
    
    return {
      deltaY,
      deltaX,
      velocityY: this.scrollVelocityY,
      isScrolling: this.isScrolling,
      direction: deltaY > 0 ? 'down' : deltaY < 0 ? 'up' : 'none'
    };
  }

  reset() {
    this.lastScrollTop = 0;
    this.lastScrollLeft = 0;
    this.scrollVelocityY = 0;
    this.isScrolling = false;
    if (this.scrollEndTimeout) {
      clearTimeout(this.scrollEndTimeout);
      this.scrollEndTimeout = null;
    }
  }
}

function tokenizeJSON(code: string): { tokens: Token[]; lineTokens: Map<number, Token[]> } {
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

  return { tokens, lineTokens };
}

export default function HighPerformanceCanvasHighlighter({
  code,
  language,
  scrollTop,
  scrollLeft,
  fontSize,
  lineHeight
}: HighPerformanceCanvasHighlighterProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const charWidthCacheRef = useRef<CharWidthCache>(new CharWidthCache());
  const chunkCacheRef = useRef<ChunkCacheManager>(new ChunkCacheManager());
  const scrollStateRef = useRef<ScrollStateManager>(new ScrollStateManager());
  const workerRef = useRef<Worker | null>(null);
  const pendingTokenizeRef = useRef<number>(0);
  const lastCodeRef = useRef<string>('');
  const lastFontSizeRef = useRef<number>(fontSize);
  const rafIdRef = useRef<number | null>(null);
  const lastRenderTimeRef = useRef<number>(0);
  const mountedRef = useRef<boolean>(true);
  const initialRenderDoneRef = useRef<boolean>(false);
  
  const [lineTokens, setLineTokens] = useState<Map<number, Token[]>>(new Map());
  const [isTokenizing, setIsTokenizing] = useState(false);
  const [isReady, setIsReady] = useState(false);

  const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
  const colors = isDark ? TOKEN_COLORS_DARK : TOKEN_COLORS_LIGHT;
  const lineHeightPx = fontSize * lineHeight;
  const totalLines = useMemo(() => code.split('\n').length, [code]);

  useEffect(() => {
    mountedRef.current = true;
    initialRenderDoneRef.current = false;
    
    if (language === 'json' && code) {
      const lineCount = code.split('\n').length;
      
      if (lineCount > PERFORMANCE_CONFIG.WORKER_THRESHOLD && workerRef.current) {
        setIsTokenizing(true);
        const taskId = Date.now();
        pendingTokenizeRef.current = taskId;
        
        workerRef.current.postMessage({
          id: taskId,
          type: 'tokenize',
          data: { code }
        });
      } else {
        const result = tokenizeJSON(code);
        setLineTokens(result.lineTokens);
        setIsReady(true);
        initialRenderDoneRef.current = true;
      }
    }
    
    return () => {
      mountedRef.current = false;
    };
  }, []);

  useEffect(() => {
    try {
      workerRef.current = new Worker(
        new URL('../workers/syntax-worker.ts', import.meta.url),
        { type: 'module' }
      );

      workerRef.current.onmessage = (event) => {
        const { id, type, result, error } = event.data;
        
        if (!mountedRef.current) return;
        
        if (error) {
          console.error('[语法高亮Worker] 错误:', error);
          const syncResult = tokenizeJSON(code);
          setLineTokens(syncResult.lineTokens);
          setIsTokenizing(false);
          setIsReady(true);
          return;
        }

        if (id === pendingTokenizeRef.current && type === 'tokenize') {
          const newLineTokens = new Map<number, Token[]>(result.lineTokens);
          setLineTokens(newLineTokens);
          setIsTokenizing(false);
          setIsReady(true);
          chunkCacheRef.current.clear();
          initialRenderDoneRef.current = true;
        }
      };

      workerRef.current.onerror = (error) => {
        console.error('[语法高亮Worker] Worker错误:', error);
        if (mountedRef.current) {
          const syncResult = tokenizeJSON(code);
          setLineTokens(syncResult.lineTokens);
          setIsTokenizing(false);
          setIsReady(true);
        }
      };
    } catch (error) {
      console.warn('[语法高亮] Worker创建失败，使用同步模式:', error);
      const result = tokenizeJSON(code);
      setLineTokens(result.lineTokens);
      setIsReady(true);
    }

    return () => {
      if (workerRef.current) {
        workerRef.current.terminate();
        workerRef.current = null;
      }
    };
  }, []);

  useEffect(() => {
    if (language !== 'json') return;
    if (!initialRenderDoneRef.current) return;

    const codeChanged = code !== lastCodeRef.current;
    const fontSizeChanged = fontSize !== lastFontSizeRef.current;

    if (fontSizeChanged) {
      charWidthCacheRef.current.clear();
      chunkCacheRef.current.clear();
      lastFontSizeRef.current = fontSize;
    }

    if (codeChanged) {
      lastCodeRef.current = code;
      const lineCount = code.split('\n').length;

      if (lineCount > PERFORMANCE_CONFIG.WORKER_THRESHOLD && workerRef.current) {
        setIsTokenizing(true);
        const taskId = Date.now();
        pendingTokenizeRef.current = taskId;
        
        workerRef.current.postMessage({
          id: taskId,
          type: 'tokenize',
          data: { code }
        });
      } else {
        const processTokens = () => {
          if (!mountedRef.current) return;
          
          const startTime = performance.now();
          const result = tokenizeJSON(code);
          const duration = performance.now() - startTime;
          
          if (duration > 16) {
            console.log(`[语法高亮] Token解析耗时: ${duration.toFixed(2)}ms, 行数: ${lineCount}`);
          }
          
          setLineTokens(result.lineTokens);
          setIsReady(true);
          chunkCacheRef.current.clear();
        };

        if (lineCount > 500 && 'requestIdleCallback' in window) {
          (window as any).requestIdleCallback(processTokens, { timeout: 100 });
        } else {
          processTokens();
        }
      }
    }
  }, [code, language, fontSize]);

  const renderChunk = useCallback((
    chunkIndex: number,
    startLine: number,
    endLine: number,
    canvasWidth: number,
    dpr: number
  ): OffscreenCanvas | HTMLCanvasElement | null => {
    const chunkHeight = (endLine - startLine) * lineHeightPx;
    
    let offscreen: OffscreenCanvas | HTMLCanvasElement;
    try {
      offscreen = new OffscreenCanvas(
        Math.ceil(canvasWidth * dpr),
        Math.ceil(chunkHeight * dpr)
      );
    } catch {
      offscreen = document.createElement('canvas');
      offscreen.width = Math.ceil(canvasWidth * dpr);
      offscreen.height = Math.ceil(chunkHeight * dpr);
    }

    const ctx = offscreen.getContext('2d') as CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;
    if (!ctx) return null;

    ctx.scale(dpr, dpr);

    const font = `${fontSize}px Consolas, Monaco, "Courier New", monospace`;
    ctx.font = font;
    ctx.textBaseline = 'top';
    ctx.textAlign = 'left';

    charWidthCacheRef.current.setContext(ctx as CanvasRenderingContext2D, font);

    const paddingLeft = 16;
    const leadingSpace = (lineHeightPx - fontSize) / 2;
    const lines = code.split('\n');

    const indentGuideColor = isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)';
    const spaceWidth = charWidthCacheRef.current.getWidth(' ');

    for (let lineNum = startLine; lineNum < endLine && lineNum < lines.length; lineNum++) {
      const line = lines[lineNum];
      const localY = (lineNum - startLine) * lineHeightPx;

      let indentLevel = 0;
      for (let i = 0; i < line.length; i += 2) {
        if (line[i] === ' ' && line[i + 1] === ' ') {
          indentLevel++;
        } else {
          break;
        }
      }

      ctx.strokeStyle = indentGuideColor;
      ctx.lineWidth = 1;
      for (let i = 1; i <= indentLevel; i++) {
        const x = paddingLeft + (i * 2 * spaceWidth);
        ctx.beginPath();
        ctx.moveTo(x, localY);
        ctx.lineTo(x, localY + lineHeightPx);
        ctx.stroke();
      }
    }

    for (let lineNum = startLine; lineNum < endLine && lineNum < lines.length; lineNum++) {
      const tokens = lineTokens.get(lineNum);
      if (!tokens) continue;

      const localY = (lineNum - startLine) * lineHeightPx + leadingSpace;
      let currentX = paddingLeft;

      for (const token of tokens) {
        if (token.value === '\n') continue;

        ctx.fillStyle = colors[token.type] || colors.text;
        ctx.fillText(token.value, currentX, localY);
        currentX += charWidthCacheRef.current.getStringWidth(token.value);
      }
    }

    return offscreen;
  }, [code, lineTokens, fontSize, lineHeightPx, colors, isDark]);

  const preloadAdjacentChunks = useCallback((
    currentChunk: number,
    direction: 'up' | 'down' | 'none',
    canvasWidth: number,
    dpr: number
  ) => {
    const totalChunks = Math.ceil(totalLines / PERFORMANCE_CONFIG.CHUNK_SIZE);
    const chunksToPreload: number[] = [];

    if (direction === 'down') {
      for (let i = 1; i <= PERFORMANCE_CONFIG.BUFFER_CHUNKS; i++) {
        const chunk = currentChunk + i;
        if (chunk < totalChunks && !chunkCacheRef.current.getChunk(chunk)) {
          chunksToPreload.push(chunk);
        }
      }
    } else if (direction === 'up') {
      for (let i = 1; i <= PERFORMANCE_CONFIG.BUFFER_CHUNKS; i++) {
        const chunk = currentChunk - i;
        if (chunk >= 0 && !chunkCacheRef.current.getChunk(chunk)) {
          chunksToPreload.push(chunk);
        }
      }
    } else {
      if (currentChunk > 0 && !chunkCacheRef.current.getChunk(currentChunk - 1)) {
        chunksToPreload.push(currentChunk - 1);
      }
      if (currentChunk < totalChunks - 1 && !chunkCacheRef.current.getChunk(currentChunk + 1)) {
        chunksToPreload.push(currentChunk + 1);
      }
    }

    if (chunksToPreload.length > 0 && 'requestIdleCallback' in window) {
      (window as any).requestIdleCallback((deadline: IdleDeadline) => {
        for (const chunkIndex of chunksToPreload) {
          if (deadline.timeRemaining() < 5) break;
          
          const chunkStartLine = chunkIndex * PERFORMANCE_CONFIG.CHUNK_SIZE;
          const chunkEndLine = Math.min((chunkIndex + 1) * PERFORMANCE_CONFIG.CHUNK_SIZE, totalLines);
          const chunkVersion = `${chunkStartLine}-${chunkEndLine}-${fontSize}-${isDark}`;
          
          if (!chunkCacheRef.current.getChunk(chunkIndex)) {
            const chunkCanvas = renderChunk(chunkIndex, chunkStartLine, chunkEndLine, canvasWidth, dpr);
            if (chunkCanvas) {
              chunkCacheRef.current.setChunk(chunkIndex, chunkCanvas, chunkVersion);
            }
          }
        }
      }, { timeout: 200 });
    }
  }, [totalLines, fontSize, isDark, renderChunk]);

  const render = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas || lineTokens.size === 0 || !isReady) return;

    const now = performance.now();
    const timeSinceLastRender = now - lastRenderTimeRef.current;
    
    if (lastRenderTimeRef.current > 0 && timeSinceLastRender < PERFORMANCE_CONFIG.SCROLL_THROTTLE_MS) {
      return;
    }
    lastRenderTimeRef.current = now;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();

    if (rect.width === 0 || rect.height === 0) return;

    const newWidth = Math.ceil(rect.width * dpr);
    const newHeight = Math.ceil(rect.height * dpr);

    if (canvas.width !== newWidth || canvas.height !== newHeight) {
      canvas.width = newWidth;
      canvas.height = newHeight;
    }

    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, rect.width, rect.height);

    const scrollState = scrollStateRef.current.update(scrollTop, scrollLeft);

    const startLine = Math.max(0, Math.floor(scrollTop / lineHeightPx));
    const endLine = Math.min(totalLines, Math.ceil((scrollTop + rect.height) / lineHeightPx) + 1);

    const startChunk = Math.floor(startLine / PERFORMANCE_CONFIG.CHUNK_SIZE);
    const endChunk = Math.ceil(endLine / PERFORMANCE_CONFIG.CHUNK_SIZE);

    let bufferStartChunk: number;
    let bufferEndChunk: number;
    
    if (scrollState.direction === 'down') {
      bufferStartChunk = Math.max(0, startChunk - 1);
      bufferEndChunk = Math.min(Math.ceil(totalLines / PERFORMANCE_CONFIG.CHUNK_SIZE), endChunk + PERFORMANCE_CONFIG.BUFFER_CHUNKS);
    } else if (scrollState.direction === 'up') {
      bufferStartChunk = Math.max(0, startChunk - PERFORMANCE_CONFIG.BUFFER_CHUNKS);
      bufferEndChunk = Math.min(Math.ceil(totalLines / PERFORMANCE_CONFIG.CHUNK_SIZE), endChunk + 1);
    } else {
      bufferStartChunk = Math.max(0, startChunk - PERFORMANCE_CONFIG.BUFFER_CHUNKS);
      bufferEndChunk = Math.min(Math.ceil(totalLines / PERFORMANCE_CONFIG.CHUNK_SIZE), endChunk + PERFORMANCE_CONFIG.BUFFER_CHUNKS);
    }

    const maxContentWidth = Math.max(rect.width, 10000);

    for (let chunkIndex = bufferStartChunk; chunkIndex < bufferEndChunk; chunkIndex++) {
      const chunkStartLine = chunkIndex * PERFORMANCE_CONFIG.CHUNK_SIZE;
      const chunkEndLine = Math.min((chunkIndex + 1) * PERFORMANCE_CONFIG.CHUNK_SIZE, totalLines);

      const chunkVersion = `${chunkStartLine}-${chunkEndLine}-${fontSize}-${isDark}`;

      let chunkCanvas = chunkCacheRef.current.getChunk(chunkIndex);
      const cachedVersion = chunkCacheRef.current.getVersion(chunkIndex);

      if (!chunkCanvas || cachedVersion !== chunkVersion) {
        chunkCanvas = renderChunk(chunkIndex, chunkStartLine, chunkEndLine, maxContentWidth, dpr);
        if (chunkCanvas) {
          chunkCacheRef.current.setChunk(chunkIndex, chunkCanvas, chunkVersion);
        }
      }

      if (chunkCanvas) {
        const chunkY = chunkStartLine * lineHeightPx - scrollTop;
        const chunkHeight = (chunkEndLine - chunkStartLine) * lineHeightPx;

        if (chunkY + chunkHeight > 0 && chunkY < rect.height) {
          const sourceX = scrollLeft * dpr;
          const sourceY = 0;
          const sourceWidth = Math.min(rect.width * dpr, chunkCanvas.width - sourceX);
          const sourceHeight = chunkCanvas.height;

          const destX = 0;
          const destY = chunkY;
          const destWidth = sourceWidth / dpr;
          const destHeight = chunkHeight;

          if (sourceWidth > 0 && sourceHeight > 0) {
            ctx.drawImage(
              chunkCanvas as CanvasImageSource,
              Math.max(0, sourceX),
              sourceY,
              sourceWidth,
              sourceHeight,
              destX,
              destY,
              destWidth,
              destHeight
            );
          }
        }
      }
    }

    // 预加载
    preloadAdjacentChunks(startChunk, scrollState.direction, maxContentWidth, dpr);
  }, [lineTokens, scrollTop, scrollLeft, fontSize, lineHeightPx, totalLines, colors, isDark, renderChunk, preloadAdjacentChunks, isReady]);

  useEffect(() => {
    if (rafIdRef.current) {
      cancelAnimationFrame(rafIdRef.current);
    }

    rafIdRef.current = requestAnimationFrame(render);

    return () => {
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current);
      }
    };
  }, [render]);

  // 监听窗口大小
  useEffect(() => {
    const handleResize = () => {
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current);
      }
      rafIdRef.current = requestAnimationFrame(render);
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [render]);

  // 清理
  useEffect(() => {
    return () => {
      mountedRef.current = false;
      charWidthCacheRef.current.clear();
      chunkCacheRef.current.clear();
      scrollStateRef.current.reset();
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current);
      }
    };
  }, []);

  useEffect(() => {
    if (isReady && canvasRef.current) {
      lastRenderTimeRef.current = 0;
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current);
      }
      rafIdRef.current = requestAnimationFrame(render);
    }
  }, [isReady, render]);

  if (language !== 'json') {
    return null;
  }

  return (
    <canvas
      ref={canvasRef}
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        pointerEvents: 'none',
        zIndex: 1,
      }}
    />
  );
}