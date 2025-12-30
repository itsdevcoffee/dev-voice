import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from './stores/appStore';
import Dashboard from './components/Dashboard';
import './styles/globals.css';

function App() {
  const { setDaemonStatus } = useAppStore();

  useEffect(() => {
    // Check daemon status on mount
    const checkStatus = async () => {
      try {
        const status = await invoke('get_daemon_status');
        setDaemonStatus(status as any);
      } catch (error) {
        console.error('Failed to get daemon status:', error);
      }
    };

    checkStatus();
    // Poll every 2 seconds
    const interval = setInterval(checkStatus, 2000);

    return () => clearInterval(interval);
  }, [setDaemonStatus]);

  return (
    <div className="min-h-screen bg-[#0a0a0a] grid-background">
      <Dashboard />
    </div>
  );
}

export default App;
