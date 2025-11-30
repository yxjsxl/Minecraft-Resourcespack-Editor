import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './AudioHoverPlayer.css';

interface AudioHoverPlayerProps {
  audioPath: string;
  position: { x: number; y: number };
  onClose: () => void;
}

export default function AudioHoverPlayer({ audioPath, position, onClose }: AudioHoverPlayerProps) {
  const [audioUrl, setAudioUrl] = useState<string>('');
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(0.7);
  const audioRef = useRef<HTMLAudioElement>(null);
  const playerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let blobUrl: string | null = null;

    const loadAudio = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const packDir = await invoke<string>('get_current_pack_path');
        
        const extensions = ['ogg', 'wav'];
        let found = false;

        for (const ext of extensions) {
          const fullPath = `${packDir}/assets/minecraft/sounds/${audioPath}.${ext}`;
          
          try {
            const exists = await invoke<boolean>('check_file_exists', { filePath: fullPath });
            
            if (exists) {
              const base64Content = await invoke<string>('read_file_as_base64', { filePath: fullPath });
              
              const byteCharacters = atob(base64Content);
              const byteNumbers = new Array(byteCharacters.length);
              for (let i = 0; i < byteCharacters.length; i++) {
                byteNumbers[i] = byteCharacters.charCodeAt(i);
              }
              const byteArray = new Uint8Array(byteNumbers);
              const blob = new Blob([byteArray], { type: `audio/${ext}` });
              
              blobUrl = URL.createObjectURL(blob);
              setAudioUrl(blobUrl);
              found = true;
              break;
            }
          } catch (e) {
            continue;
          }
        }

        if (!found) {
          setError('音频文件不存在');
        }
      } catch (err) {
        console.error('加载音频失败:', err);
        setError('加载失败');
      } finally {
        setIsLoading(false);
      }
    };

    loadAudio();

    return () => {
      if (blobUrl) {
        URL.revokeObjectURL(blobUrl);
      }
    };
  }, [audioPath]);

  useEffect(() => {
    if (audioRef.current) {
      audioRef.current.volume = volume;
    }
  }, [volume]);

  const togglePlay = () => {
    if (!audioRef.current || error) return;

    if (isPlaying) {
      audioRef.current.pause();
    } else {
      audioRef.current.play().catch(err => {
        console.error('播放失败:', err);
        setError('播放失败');
      });
    }
    setIsPlaying(!isPlaying);
  };

  const handleTimeUpdate = () => {
    if (audioRef.current && audioRef.current.duration) {
      setProgress((audioRef.current.currentTime / audioRef.current.duration) * 100);
    }
  };

  const handleLoadedMetadata = () => {
    if (audioRef.current) {
      setDuration(audioRef.current.duration);
    }
  };

  const handleEnded = () => {
    setIsPlaying(false);
    setProgress(0);
  };

  const handleError = () => {
    setError('播放失败');
    setIsPlaying(false);
  };

  const formatTime = (seconds: number): string => {
    if (!seconds || isNaN(seconds)) return '0:00';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const handleProgressClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!audioRef.current || !duration) return;
    
    const rect = e.currentTarget.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const percentage = clickX / rect.width;
    const newTime = percentage * duration;
    
    audioRef.current.currentTime = newTime;
    setProgress(percentage * 100);
  };

  return (
    <div 
      ref={playerRef}
      className="audio-hover-player"
      style={{
        top: `${position.y}px`,
        left: `${position.x}px`,
      }}
      onMouseLeave={onClose}
    >
      <div className="audio-hover-header">
        <div className="audio-path-display">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M9 18V5l12-2v13"></path>
            <circle cx="6" cy="18" r="3"></circle>
            <circle cx="18" cy="16" r="3"></circle>
          </svg>
          <span>{audioPath}</span>
        </div>
        <button className="close-btn" onClick={onClose} title="关闭">
          x
        </button>
      </div>

      {isLoading && (
        <div className="audio-hover-loading">
          <div className="spinner-small"></div>
          <span>加载中...</span>
        </div>
      )}

      {error && (
        <div className="audio-hover-error" title={error}>
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          <span>{error}</span>
        </div>
      )}

      {!isLoading && !error && audioUrl && (
        <div className="audio-hover-controls">
          <audio
            ref={audioRef}
            src={audioUrl}
            onTimeUpdate={handleTimeUpdate}
            onLoadedMetadata={handleLoadedMetadata}
            onEnded={handleEnded}
            onError={handleError}
          />

          <div className="play-controls">
            <button 
              className="play-btn-hover" 
              onClick={togglePlay}
              title={isPlaying ? '暂停' : '播放'}
            >
              {isPlaying ? (
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                  <rect x="6" y="4" width="4" height="16"></rect>
                  <rect x="14" y="4" width="4" height="16"></rect>
                </svg>
              ) : (
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                  <polygon points="5 3 19 12 5 21 5 3"></polygon>
                </svg>
              )}
            </button>

            <div className="progress-section">
              <div 
                className="progress-bar-hover" 
                onClick={handleProgressClick}
                title="点击跳转"
              >
                <div className="progress-fill-hover" style={{ width: `${progress}%` }} />
              </div>
              <div className="time-display-hover">
                {formatTime((progress / 100) * duration)} / {formatTime(duration)}
              </div>
            </div>
          </div>

          <div className="volume-section">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
              <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
            </svg>
            <input
              type="range"
              min="0"
              max="1"
              step="0.1"
              value={volume}
              onChange={(e) => setVolume(parseFloat(e.target.value))}
              className="volume-slider-hover"
              title={`音量: ${Math.round(volume * 100)}%`}
            />
          </div>
        </div>
      )}
    </div>
  );
}