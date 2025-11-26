import { useState } from "react";
import "./PngCreatorDialog.css";

interface PngCreatorDialogProps {
  onClose: () => void;
  onConfirm: (width: number, height: number, fileName: string) => void;
  folderPath: string;
}

export default function PngCreatorDialog({ onClose, onConfirm, folderPath }: PngCreatorDialogProps) {
  const [fileName, setFileName] = useState<string>("new_image.png");
  const [size, setSize] = useState<number>(16);

  // 选项: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192
  const sizeOptions = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];

  const handleConfirm = () => {
    if (!fileName.trim()) {
      alert("请输入文件名");
      return;
    }

    // 确保文件名以.png结尾
    const finalFileName = fileName.endsWith('.png') ? fileName : `${fileName}.png`;
    
    onConfirm(size, size, finalFileName);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleConfirm();
    } else if (e.key === 'Escape') {
      onClose();
    }
  };

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-content png-creator-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h3>新增PNG图片</h3>
          <button className="dialog-close" onClick={onClose}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        
        <div className="dialog-body">
          <div className="form-group">
            <label>文件名:</label>
            <input
              type="text"
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="输入文件名"
              autoFocus
            />
          </div>

          <div className="form-group">
            <label>尺寸 (像素):</label>
            <select
              value={size}
              onChange={(e) => setSize(Number(e.target.value))}
            >
              {sizeOptions.map(option => (
                <option key={`size-${option}`} value={option}>{option}px | {option}px</option>
              ))}
            </select>
          </div>

          <div className="preview-section">
            <h4>预览</h4>
            <div className="preview-container">
              <div
                className="preview-canvas"
                style={{
                  width: `${Math.min(size, 200)}px`,
                  height: `${Math.min(size, 200)}px`
                }}
              >
                <span className="preview-size">{size} X {size}</span>
              </div>
            </div>
          </div>

          <div className="form-info">
            <p> 保存位置: {folderPath || '根目录'}</p>
            <p> 分辨率: {size} | {size}</p>
            <p> 图片将创建为正方形透明PNG格式</p>
          </div>
        </div>

        <div className="dialog-footer">
          <button className="btn-secondary" onClick={onClose}>
            取消
          </button>
          <button className="btn-primary" onClick={handleConfirm}>
            创建
          </button>
        </div>
      </div>
    </div>
  );
}