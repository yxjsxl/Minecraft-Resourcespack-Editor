import { getCurrentWindow } from '@tauri-apps/api/window';
import { useState, useEffect } from 'react';
import './TitleBar.css';
import logoImg from '../assets/logo.png';

interface TitleBarProps {
  packSize?: number;      // 材质包大小
  historySize?: number;   // 历史记录大小
  showStats?: boolean;    // 是否显示统计信息
}

const TitleBar = ({ packSize = 0, historySize = 0, showStats = false }: TitleBarProps) => {
  const [isMaximized, setIsMaximized] = useState(false);
  const appWindow = getCurrentWindow();

  // 格式化文件大小
  const formatSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
  };

  useEffect(() => {
    const checkMaximized = async () => {
      const maximized = await appWindow.isMaximized();
      setIsMaximized(maximized);
    };

    checkMaximized();

    // 监听窗口状态变化
    const unlisten = appWindow.onResized(() => {
      checkMaximized();
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const handleMinimize = async () => {
    await appWindow.minimize();
  };

  const handleMaximize = async () => {
    await appWindow.toggleMaximize();
    // 等待状态更新后再检查
    setTimeout(async () => {
      const maximized = await appWindow.isMaximized();
      setIsMaximized(maximized);
    }, 100);
  };

  const handleClose = async () => {
    await appWindow.close();
  };

  return (
    <div className="titlebar" data-tauri-drag-region>
      <div className="titlebar-left" data-tauri-drag-region>
        <img src={logoImg} alt="Logo" className="titlebar-icon" />
        <div className="titlebar-text">
          <span className="titlebar-title">Minecraft 材质包编辑器</span>
          <span className="titlebar-subtitle">Powered By Little_100</span>
        </div>
      </div>
      
      {showStats && (
        <div className="titlebar-center" data-tauri-drag-region>
          <div className="size-stats">
            <span className="stat-item">
              <span className="stat-label">材质包:</span>
              <span className="stat-value">{formatSize(packSize)}</span>
            </span>
            <span className="stat-divider">|</span>
            <span className="stat-item">
              <span className="stat-label">历史记录:</span>
              <span className="stat-value">{formatSize(historySize)}</span>
            </span>
          </div>
        </div>
      )}
      
      <div className="titlebar-controls">
        <button
          className="titlebar-button minimize"
          onClick={handleMinimize}
          title="最小化"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect x="0" y="5" width="12" height="2" fill="currentColor" />
          </svg>
        </button>
        <button
          className="titlebar-button maximize"
          onClick={handleMaximize}
          title={isMaximized ? "还原" : "最大化"}
        >
          {isMaximized ? (
            <svg width="12" height="12" viewBox="0 0 12 12">
              <rect x="2" y="0" width="10" height="10" fill="none" stroke="currentColor" strokeWidth="1.5" />
              <rect x="0" y="2" width="10" height="10" fill="none" stroke="currentColor" strokeWidth="1.5" />
            </svg>
          ) : (
            <svg width="12" height="12" viewBox="0 0 12 12">
              <rect x="0" y="0" width="12" height="12" fill="none" stroke="currentColor" strokeWidth="1.5" />
            </svg>
          )}
        </button>
        <button
          className="titlebar-button close"
          onClick={handleClose}
          title="关闭"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M0 0 L12 12 M12 0 L0 12" stroke="currentColor" strokeWidth="1.5" />
          </svg>
        </button>
      </div>
    </div>
  );
};

export default TitleBar;