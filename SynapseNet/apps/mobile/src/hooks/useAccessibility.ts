import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface AccessibilityInfo {
  is_voiceover_running: boolean;
  is_talkback_running: boolean;
  preferred_content_size: string;
  is_bold_text_enabled: boolean;
  is_reduce_motion_enabled: boolean;
}

export function useAccessibility() {
  const [info, setInfo] = useState<AccessibilityInfo | null>(null);
  const [isScreenReaderRunning, setIsScreenReaderRunning] = useState(false);

  useEffect(() => {
    loadAccessibilityInfo();
  }, []);

  const loadAccessibilityInfo = async () => {
    try {
      const [accessibilityInfo, screenReaderStatus] = await Promise.all([
        invoke<AccessibilityInfo>('get_accessibility_info'),
        invoke<boolean>('is_screen_reader_running'),
      ]);
      
      setInfo(accessibilityInfo);
      setIsScreenReaderRunning(screenReaderStatus);
    } catch (error) {
      console.error('Failed to load accessibility info:', error);
    }
  };

  const announce = async (message: string) => {
    try {
      await invoke('announce_for_accessibility', { message });
    } catch (error) {
      console.error('Failed to announce:', error);
    }
  };

  const postNotification = async (notificationType: string, message: string) => {
    try {
      await invoke('post_accessibility_notification', {
        notificationType,
        message,
      });
    } catch (error) {
      console.error('Failed to post notification:', error);
    }
  };

  return {
    info,
    isScreenReaderRunning,
    announce,
    postNotification,
    reload: loadAccessibilityInfo,
  };
}
