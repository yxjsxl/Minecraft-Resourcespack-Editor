import { useRef, useEffect, useCallback, useState } from 'react';
import Editor, { OnMount, OnChange, loader } from '@monaco-editor/react';
import type { editor } from 'monaco-editor';

loader.config({
  paths: {
    vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs'
  }
});

interface MonacoEditorProps {
  content: string;
  filePath: string;
  onChange?: (content: string) => void;
  onSave?: () => void;
  readOnly?: boolean;
  initialLine?: number;
  fontSize?: number;
  wordWrap?: boolean;
}

function getLanguageFromPath(filePath: string): string {
  const ext = filePath.toLowerCase().split('.').pop();
  switch (ext) {
    case 'json':
    case 'mcmeta':
      return 'json';
    case 'js':
      return 'javascript';
    case 'ts':
      return 'typescript';
    case 'html':
      return 'html';
    case 'css':
      return 'css';
    case 'md':
      return 'markdown';
    case 'xml':
      return 'xml';
    case 'yaml':
    case 'yml':
      return 'yaml';
    case 'properties':
    case 'lang':
      return 'ini';
    default:
      return 'plaintext';
  }
}

export default function MonacoEditor({
  content,
  filePath,
  onChange,
  onSave,
  readOnly = false,
  initialLine,
  fontSize = 13,
  wordWrap = false
}: MonacoEditorProps) {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const [isEditorReady, setIsEditorReady] = useState(false);
  const isDark = document.documentElement.getAttribute('data-theme') === 'dark';

  const handleEditorMount: OnMount = useCallback((editor, monaco) => {
    editorRef.current = editor;
    setIsEditorReady(true);

    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
      if (onSave) {
        onSave();
      }
    });

    if (initialLine && initialLine > 0) {
      setTimeout(() => {
        editor.revealLineInCenter(initialLine);
        editor.setPosition({ lineNumber: initialLine, column: 1 });
        
        const decorations = editor.createDecorationsCollection([
          {
            range: new monaco.Range(initialLine, 1, initialLine, 1),
            options: {
              isWholeLine: true,
              className: 'monaco-highlighted-line',
              glyphMarginClassName: 'monaco-highlighted-glyph'
            }
          }
        ]);

        setTimeout(() => {
          decorations.clear();
        }, 2000);
      }, 100);
    }

    if (getLanguageFromPath(filePath) === 'json') {
      monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
        validate: true,
        allowComments: false,
        schemas: [],
        enableSchemaRequest: false
      });
    }
  }, [initialLine, onSave, filePath]);

  const handleChange: OnChange = useCallback((value) => {
    if (onChange && value !== undefined) {
      onChange(value);
    }
  }, [onChange]);

  useEffect(() => {
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.attributeName === 'data-theme') {}
      });
    });

    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme']
    });

    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    if (editorRef.current && isEditorReady) {
      const model = editorRef.current.getModel();
      if (model) {
        const currentValue = model.getValue();
        if (currentValue !== content) {
          const position = editorRef.current.getPosition();
          
          model.setValue(content);
          
          if (position) {
            editorRef.current.setPosition(position);
          }
        }
      }
    }
  }, [content, isEditorReady]);

  useEffect(() => {
    if (editorRef.current) {
      editorRef.current.updateOptions({
        fontSize,
        wordWrap: wordWrap ? 'on' : 'off',
        readOnly
      });
    }
  }, [fontSize, wordWrap, readOnly]);

  const language = getLanguageFromPath(filePath);

  return (
    <div className="monaco-editor-container">
      <Editor
        height="100%"
        language={language}
        value={content}
        theme={isDark ? 'vs-dark' : 'vs'}
        onMount={handleEditorMount}
        onChange={handleChange}
        options={{
          fontSize,
          fontFamily: "'Consolas', 'Monaco', 'Courier New', monospace",
          lineHeight: 1.5 * fontSize,
          minimap: {
            enabled: true,
            maxColumn: 80,
            renderCharacters: false
          },
          scrollBeyondLastLine: false,
          wordWrap: wordWrap ? 'on' : 'off',
          readOnly,
          automaticLayout: true,
          tabSize: 2,
          insertSpaces: true,
          renderWhitespace: 'selection',
          bracketPairColorization: {
            enabled: true
          },
          guides: {
            indentation: true,
            bracketPairs: true
          },
          folding: true,
          foldingStrategy: 'indentation',
          showFoldingControls: 'mouseover',
          lineNumbers: 'on',
          glyphMargin: false,
          lineDecorationsWidth: 10,
          lineNumbersMinChars: 4,
          renderLineHighlight: 'line',
          scrollbar: {
            vertical: 'auto',
            horizontal: 'auto',
            verticalScrollbarSize: 10,
            horizontalScrollbarSize: 10
          },
          overviewRulerBorder: false,
          hideCursorInOverviewRuler: true,
          contextmenu: true,
          mouseWheelZoom: true,
          smoothScrolling: true,
          cursorBlinking: 'smooth',
          cursorSmoothCaretAnimation: 'on',
          padding: {
            top: 16,
            bottom: 16
          },
          largeFileOptimizations: true,
          quickSuggestions: false,
          suggestOnTriggerCharacters: false,
          acceptSuggestionOnEnter: 'off',
          wordBasedSuggestions: 'off',
          parameterHints: {
            enabled: false
          },
          'semanticHighlighting.enabled': true
        }}
        loading={
          <div className="monaco-loading">
            <div className="monaco-loading-spinner"></div>
            <span>加载编辑器...</span>
          </div>
        }
      />
    </div>
  );
}